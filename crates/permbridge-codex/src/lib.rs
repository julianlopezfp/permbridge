//! Read-only inspection of selected Codex configuration files.
//!
//! File observations describe declared settings, not a running Codex session.
//! CLI overrides, managed requirements, trust, and permission profiles can
//! change the active posture. See the Codex compatibility guide for scope.

mod config;

use std::fmt;
use std::io;
use std::path::PathBuf;

use permbridge_core::{
    AgentAdapter, CanonicalPolicy, Capability, CapabilityObservation, Decision, EffectivePosture,
    EnforcementStrength,
};

use config::{ApprovalMode, ConfigSnapshot, SandboxMode, Sourced};

/// The configuration layer from which an explicit setting was read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigSource {
    /// The supplied Codex home directory's `config.toml`.
    User,
    /// The supplied workspace root's `.codex/config.toml`.
    Project,
}

/// Whether an inspected configuration source was available.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceStatus {
    /// The file was read and parsed.
    Read,
    /// No file was present at the expected path.
    Missing,
    /// No project root was supplied.
    NotProvided,
    /// The caller did not mark the project trusted, so its file was not read.
    SkippedUntrusted,
}

/// Supported setting names in the static inspection report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexSetting {
    /// Legacy local command sandbox mode.
    SandboxMode,
    /// Command approval policy.
    ApprovalPolicy,
    /// Network toggle for the workspace-write command sandbox.
    WorkspaceNetworkAccess,
    /// New permission profiles, which this adapter does not interpret.
    PermissionProfile,
}

/// Interpretation of one selected setting, without exposing unrelated config.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingState {
    /// No inspected file explicitly sets this value.
    NotConfigured,
    /// A recognized value was read; it is a declaration, not runtime evidence.
    Configured(&'static str),
    /// A recognized configuration feature is outside this adapter's mapping.
    Unsupported,
    /// A value exists but is not recognized by this adapter version.
    Unknown,
}

/// A selected setting and the file layer that supplied it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SettingObservation {
    /// The inspected setting.
    pub setting: CodexSetting,
    /// Its normalized state.
    pub state: SettingState,
    /// Highest-priority inspected source with an explicit value, if any.
    pub source: Option<ConfigSource>,
}

/// File-level evidence alongside the canonical capability observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexInspection {
    /// Conservative canonical observations derived from inspected files.
    pub posture: EffectivePosture,
    /// Selected Codex settings and their provenance.
    pub settings: Vec<SettingObservation>,
    /// User configuration file status.
    pub user_source: SourceStatus,
    /// Project configuration file status.
    pub project_source: SourceStatus,
}

/// A read or parse failure in a supported configuration source.
#[derive(Debug)]
pub enum CodexError {
    /// Reading a configuration file failed for a reason other than absence.
    Read {
        /// Source that failed.
        source: ConfigSource,
        /// I/O error category. File paths and contents are intentionally omitted.
        kind: io::ErrorKind,
    },
    /// The source is not valid TOML.
    InvalidToml {
        /// Source that failed.
        source: ConfigSource,
    },
    /// A supported setting has an invalid TOML type or retired value.
    InvalidSetting {
        /// Source that failed.
        source: ConfigSource,
        /// Stable setting name; no raw value is included.
        setting: &'static str,
    },
}

impl fmt::Display for CodexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { source, kind } => write!(f, "cannot read {source:?} Codex config: {kind}"),
            Self::InvalidToml { source } => write!(f, "invalid TOML in {source:?} Codex config"),
            Self::InvalidSetting { source, setting } => {
                write!(f, "invalid {setting} in {source:?} Codex config")
            }
        }
    }
}

impl std::error::Error for CodexError {}

/// Explicit roots and trust state for reproducible, read-only inspection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexConfigPaths {
    /// Directory containing user `config.toml`.
    pub codex_home: PathBuf,
    /// Workspace whose root `.codex/config.toml` may be read.
    pub workspace_root: Option<PathBuf>,
    /// Whether the caller has established project trust for this workspace.
    pub project_trusted: bool,
}

impl CodexConfigPaths {
    /// Construct roots without consulting environment variables or user files.
    #[must_use]
    pub fn new(
        codex_home: PathBuf,
        workspace_root: Option<PathBuf>,
        project_trusted: bool,
    ) -> Self {
        Self {
            codex_home,
            workspace_root,
            project_trusted,
        }
    }
}

/// Inspects selected Codex config files without launching or changing Codex.
pub struct CodexAdapter {
    paths: CodexConfigPaths,
}

impl CodexAdapter {
    /// Create an adapter with caller-supplied configuration roots and trust.
    #[must_use]
    pub fn new(paths: CodexConfigPaths) -> Self {
        Self { paths }
    }

    /// Return canonical observations and source-level evidence.
    ///
    /// The result is limited to inspected files. It cannot establish runtime
    /// overrides, managed requirements, or OS sandbox enforcement.
    pub fn inspect_report(&self, desired: &CanonicalPolicy) -> Result<CodexInspection, CodexError> {
        let snapshot = config::load(&self.paths)?;
        let posture = normalize(&snapshot, desired);
        let settings = vec![
            setting(
                CodexSetting::SandboxMode,
                snapshot.sandbox,
                |value| match value {
                    SandboxMode::ReadOnly => SettingState::Configured("read-only"),
                    SandboxMode::WorkspaceWrite => SettingState::Configured("workspace-write"),
                    SandboxMode::DangerFullAccess => SettingState::Configured("danger-full-access"),
                    SandboxMode::Unknown => SettingState::Unknown,
                },
            ),
            setting(
                CodexSetting::ApprovalPolicy,
                snapshot.approval,
                |value| match value {
                    ApprovalMode::OnRequest => SettingState::Configured("on-request"),
                    ApprovalMode::Never => SettingState::Configured("never"),
                    ApprovalMode::Granular => SettingState::Unsupported,
                    ApprovalMode::Unknown => SettingState::Unknown,
                },
            ),
            setting(
                CodexSetting::WorkspaceNetworkAccess,
                snapshot.network,
                |value| SettingState::Configured(if value { "true" } else { "false" }),
            ),
            SettingObservation {
                setting: CodexSetting::PermissionProfile,
                state: if snapshot.permission_profile.is_some() {
                    SettingState::Unsupported
                } else {
                    SettingState::NotConfigured
                },
                source: snapshot.permission_profile,
            },
        ];

        Ok(CodexInspection {
            posture,
            settings,
            user_source: snapshot.user_source,
            project_source: snapshot.project_source,
        })
    }
}

impl AgentAdapter for CodexAdapter {
    type Error = CodexError;

    fn id(&self) -> &str {
        "codex"
    }

    fn inspect(&self, desired: &CanonicalPolicy) -> Result<EffectivePosture, Self::Error> {
        self.inspect_report(desired).map(|report| report.posture)
    }
}

fn setting<T: Copy>(
    name: CodexSetting,
    value: Option<Sourced<T>>,
    state: impl FnOnce(T) -> SettingState,
) -> SettingObservation {
    match value {
        Some(value) => SettingObservation {
            setting: name,
            state: state(value.value),
            source: Some(value.source),
        },
        None => SettingObservation {
            setting: name,
            state: SettingState::NotConfigured,
            source: None,
        },
    }
}

fn normalize(snapshot: &ConfigSnapshot, desired: &CanonicalPolicy) -> EffectivePosture {
    use Capability::{
        ExecutionCommand, ExecutionDefault, FilesystemOutsideWorkspace, FilesystemRead,
        FilesystemWrite, NetworkDefault, NetworkDomain,
    };
    use CapabilityObservation::{Ambiguous, NotConfigured, Unsupported};

    let mut posture = EffectivePosture::default();
    let sandbox = snapshot.sandbox.map(|value| value.value);
    let profile = snapshot.permission_profile.is_some();
    let declared_allow = || CapabilityObservation::Known {
        decision: Decision::Allow,
        enforcement: EnforcementStrength::Declared,
    };

    let filesystem_read = if profile {
        Ambiguous
    } else {
        match sandbox {
            Some(
                SandboxMode::ReadOnly | SandboxMode::WorkspaceWrite | SandboxMode::DangerFullAccess,
            ) => declared_allow(),
            Some(SandboxMode::Unknown) => Ambiguous,
            None => NotConfigured,
        }
    };
    let filesystem_write = if profile {
        Ambiguous
    } else {
        match sandbox {
            Some(SandboxMode::WorkspaceWrite | SandboxMode::DangerFullAccess) => declared_allow(),
            Some(SandboxMode::ReadOnly | SandboxMode::Unknown) => Ambiguous,
            None => NotConfigured,
        }
    };
    let outside_workspace = if profile {
        Ambiguous
    } else {
        match sandbox {
            Some(SandboxMode::DangerFullAccess) => declared_allow(),
            Some(_) => Ambiguous,
            None => NotConfigured,
        }
    };

    posture.capabilities.insert(FilesystemRead, filesystem_read);
    posture
        .capabilities
        .insert(FilesystemWrite, filesystem_write);
    posture
        .capabilities
        .insert(FilesystemOutsideWorkspace, outside_workspace);
    posture.capabilities.insert(
        NetworkDefault,
        if sandbox.is_none() && snapshot.network.is_none() && !profile {
            NotConfigured
        } else {
            Ambiguous
        },
    );
    posture.capabilities.insert(
        ExecutionDefault,
        if snapshot.approval.is_none() && sandbox.is_none() && !profile {
            NotConfigured
        } else {
            Ambiguous
        },
    );

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
