//! Read-only inspection of selected Claude Code JSON settings files.
//!
//! The report describes file declarations, not an active Claude Code session.
//! It does not execute hooks or commands, inspect managed policy, or establish
//! runtime enforcement.

mod config;

use std::fmt;
use std::io;
use std::path::PathBuf;

use permbridge_core::{
    AgentAdapter, CanonicalPolicy, Capability, CapabilityObservation, EffectivePosture,
};

use config::{ConfigSnapshot, PermissionMode, Sourced};

/// One inspected settings layer, ordered from user to project-local scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigSource {
    /// The supplied configuration directory's `settings.json`.
    User,
    /// The supplied workspace root's `.claude/settings.json`.
    Project,
    /// The supplied workspace root's `.claude/settings.local.json`.
    Local,
}

/// Whether an optional settings file was inspected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceStatus {
    /// The file was read and parsed.
    Read,
    /// The expected file was absent.
    Missing,
    /// The caller did not supply a workspace root.
    NotProvided,
    /// The caller did not mark the workspace trusted.
    SkippedUntrusted,
}

/// Selected security-relevant settings; raw rule and path values are withheld.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeSetting {
    /// Session starting permission mode.
    PermissionMode,
    /// Tool allow rules.
    AllowRules,
    /// Tool ask rules.
    AskRules,
    /// Tool deny rules.
    DenyRules,
    /// Paths that expand the working directory boundary.
    AdditionalDirectories,
    /// Whether the shell-command sandbox is requested.
    SandboxEnabled,
    /// Whether startup fails when the sandbox is unavailable.
    SandboxFailIfUnavailable,
    /// Whether a command can request an unsandboxed retry.
    SandboxAllowUnsandboxedCommands,
    /// Sandbox filesystem options, not normalized by this adapter.
    SandboxFilesystem,
    /// Sandbox network options, not normalized by this adapter.
    SandboxNetwork,
    /// Hooks, whose effect depends on executable code not inspected here.
    Hooks,
}

/// Interpretation of one selected setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingState {
    /// No inspected file contains this setting.
    NotConfigured,
    /// A documented value or setting presence was found in a file.
    Configured(&'static str),
    /// A present setting is outside this adapter's supported mapping.
    Unsupported,
    /// A string value is not recognized by this adapter version.
    Unknown,
}

/// A setting and every inspected source that contributes to it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingObservation {
    /// The inspected setting.
    pub setting: ClaudeSetting,
    /// Its file-level interpretation.
    pub state: SettingState,
    /// Selected scalar source or merged list contributors, without file paths.
    pub sources: Vec<ConfigSource>,
}

/// Canonical observations and source-level evidence from selected files.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeInspection {
    /// Conservative capability observations, without a compliance judgment.
    pub posture: EffectivePosture,
    /// Selected setting observations and provenance.
    pub settings: Vec<SettingObservation>,
    /// User settings file status.
    pub user_source: SourceStatus,
    /// Shared project settings file status.
    pub project_source: SourceStatus,
    /// Project-local settings file status.
    pub local_source: SourceStatus,
}

/// A supported source could not be read or parsed safely.
#[derive(Debug)]
pub enum ClaudeError {
    /// Reading failed for a reason other than file absence.
    Read {
        /// Source layer that failed.
        source: ConfigSource,
        /// I/O category without a path or underlying error text.
        kind: io::ErrorKind,
    },
    /// A settings file is not valid JSON.
    InvalidJson {
        /// Source layer that failed.
        source: ConfigSource,
    },
    /// A supported setting has an invalid JSON type.
    InvalidSetting {
        /// Source layer that failed.
        source: ConfigSource,
        /// Stable setting name, never the raw value.
        setting: &'static str,
    },
}

impl fmt::Display for ClaudeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { source, kind } => {
                write!(f, "cannot read {source:?} Claude Code settings: {kind}")
            }
            Self::InvalidJson { source } => {
                write!(f, "invalid JSON in {source:?} Claude Code settings")
            }
            Self::InvalidSetting { source, setting } => {
                write!(f, "invalid {setting} in {source:?} Claude Code settings")
            }
        }
    }
}

impl std::error::Error for ClaudeError {}

/// Injected roots and trust state for deterministic, read-only inspection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeConfigPaths {
    /// Directory containing user `settings.json` (normally `~/.claude`).
    pub config_dir: PathBuf,
    /// Workspace root whose `.claude` files may be inspected.
    pub workspace_root: Option<PathBuf>,
    /// Whether the caller has established trust for the workspace.
    pub project_trusted: bool,
}

impl ClaudeConfigPaths {
    /// Construct paths without consulting the process environment.
    #[must_use]
    pub fn new(
        config_dir: PathBuf,
        workspace_root: Option<PathBuf>,
        project_trusted: bool,
    ) -> Self {
        Self {
            config_dir,
            workspace_root,
            project_trusted,
        }
    }
}

/// Inspects selected Claude Code settings files without launching Claude Code.
pub struct ClaudeCodeAdapter {
    paths: ClaudeConfigPaths,
}

impl ClaudeCodeAdapter {
    /// Create an adapter with caller-supplied roots and trust.
    #[must_use]
    pub fn new(paths: ClaudeConfigPaths) -> Self {
        Self { paths }
    }

    /// Return conservative canonical observations and sanitized file evidence.
    pub fn inspect_report(
        &self,
        desired: &CanonicalPolicy,
    ) -> Result<ClaudeInspection, ClaudeError> {
        let snapshot = config::load(&self.paths)?;
        let posture = normalize(&snapshot, desired);
        let settings = vec![
            scalar(
                ClaudeSetting::PermissionMode,
                snapshot.mode,
                |mode| match mode {
                    PermissionMode::Default => SettingState::Configured("default"),
                    PermissionMode::AcceptEdits => SettingState::Configured("acceptEdits"),
                    PermissionMode::Plan => SettingState::Configured("plan"),
                    PermissionMode::Auto => SettingState::Configured("auto"),
                    PermissionMode::DontAsk => SettingState::Configured("dontAsk"),
                    PermissionMode::BypassPermissions => {
                        SettingState::Configured("bypassPermissions")
                    }
                    PermissionMode::Unknown => SettingState::Unknown,
                },
            ),
            presence(ClaudeSetting::AllowRules, &snapshot.allow, false),
            presence(ClaudeSetting::AskRules, &snapshot.ask, false),
            presence(ClaudeSetting::DenyRules, &snapshot.deny, false),
            presence(
                ClaudeSetting::AdditionalDirectories,
                &snapshot.additional_directories,
                true,
            ),
            scalar(
                ClaudeSetting::SandboxEnabled,
                snapshot.sandbox_enabled,
                boolean,
            ),
            scalar(
                ClaudeSetting::SandboxFailIfUnavailable,
                snapshot.fail_if_unavailable,
                boolean,
            ),
            scalar(
                ClaudeSetting::SandboxAllowUnsandboxedCommands,
                snapshot.allow_unsandboxed_commands,
                boolean,
            ),
            presence(
                ClaudeSetting::SandboxFilesystem,
                &snapshot.sandbox_filesystem,
                true,
            ),
            presence(
                ClaudeSetting::SandboxNetwork,
                &snapshot.sandbox_network,
                true,
            ),
            presence(ClaudeSetting::Hooks, &snapshot.hooks, true),
        ];
        Ok(ClaudeInspection {
            posture,
            settings,
            user_source: snapshot.user_source,
            project_source: snapshot.project_source,
            local_source: snapshot.local_source,
        })
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    type Error = ClaudeError;

    fn id(&self) -> &str {
        "claude-code"
    }

    fn inspect(&self, desired: &CanonicalPolicy) -> Result<EffectivePosture, Self::Error> {
        self.inspect_report(desired).map(|report| report.posture)
    }
}

fn boolean(value: bool) -> SettingState {
    SettingState::Configured(if value { "true" } else { "false" })
}

fn scalar<T: Copy>(
    setting: ClaudeSetting,
    value: Option<Sourced<T>>,
    state: impl FnOnce(T) -> SettingState,
) -> SettingObservation {
    match value {
        Some(value) => SettingObservation {
            setting,
            state: state(value.value),
            sources: vec![value.source],
        },
        None => SettingObservation {
            setting,
            state: SettingState::NotConfigured,
            sources: Vec::new(),
        },
    }
}

fn presence(
    setting: ClaudeSetting,
    sources: &[ConfigSource],
    unsupported: bool,
) -> SettingObservation {
    let state = if sources.is_empty() {
        SettingState::NotConfigured
    } else if unsupported {
        SettingState::Unsupported
    } else {
        SettingState::Configured("rule list configured")
    };
    SettingObservation {
        setting,
        state,
        sources: sources.to_vec(),
    }
}

fn normalize(snapshot: &ConfigSnapshot, desired: &CanonicalPolicy) -> EffectivePosture {
    use Capability::{
        ExecutionCommand, ExecutionDefault, FilesystemOutsideWorkspace, FilesystemRead,
        FilesystemWrite, NetworkDefault, NetworkDomain,
    };
    use CapabilityObservation::{Ambiguous, NotConfigured, Unsupported};

    let permission_evidence = snapshot.mode.is_some()
        || !snapshot.allow.is_empty()
        || !snapshot.ask.is_empty()
        || !snapshot.deny.is_empty()
        || !snapshot.hooks.is_empty();
    let sandbox_evidence = snapshot.sandbox_enabled.is_some()
        || snapshot.fail_if_unavailable.is_some()
        || snapshot.allow_unsandboxed_commands.is_some();
    let source_gap = matches!(
        snapshot.project_source,
        SourceStatus::NotProvided | SourceStatus::SkippedUntrusted
    );
    let filesystem_evidence = permission_evidence
        || !snapshot.additional_directories.is_empty()
        || sandbox_evidence
        || !snapshot.sandbox_filesystem.is_empty();
    let network_evidence =
        permission_evidence || sandbox_evidence || !snapshot.sandbox_network.is_empty();
    let execution_evidence = permission_evidence || sandbox_evidence;
    let observed = |has_evidence| {
        if has_evidence || source_gap {
            Ambiguous
        } else {
            NotConfigured
        }
    };

    let mut posture = EffectivePosture::default();
    posture
        .capabilities
        .insert(FilesystemRead, observed(filesystem_evidence));
    posture
        .capabilities
        .insert(FilesystemWrite, observed(filesystem_evidence));
    posture
        .capabilities
        .insert(FilesystemOutsideWorkspace, observed(filesystem_evidence));
    posture
        .capabilities
        .insert(NetworkDefault, observed(network_evidence));
    posture
        .capabilities
        .insert(ExecutionDefault, observed(execution_evidence));
    for rule in &desired.network.domain_rules {
        posture
            .capabilities
            .insert(NetworkDomain(rule.domain.clone()), Unsupported);
    }
    for rule in &desired.execution.rules {
        posture
            .capabilities
            .insert(ExecutionCommand(rule.command.clone()), Unsupported);
    }
    posture
}
