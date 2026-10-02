use std::fs;
use std::io;
use std::path::Path;

use toml::{Table, Value};

use crate::{CodexConfigPaths, CodexError, ConfigSource, SourceStatus};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SandboxMode {
    ReadOnly,
    WorkspaceWrite,
    DangerFullAccess,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ApprovalMode {
    OnRequest,
    Never,
    Granular,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Sourced<T> {
    pub(crate) value: T,
    pub(crate) source: ConfigSource,
}

#[derive(Default)]
struct FileConfig {
    sandbox: Option<SandboxMode>,
    approval: Option<ApprovalMode>,
    network: Option<bool>,
    permission_profile: bool,
}

pub(crate) struct ConfigSnapshot {
    pub(crate) sandbox: Option<Sourced<SandboxMode>>,
    pub(crate) approval: Option<Sourced<ApprovalMode>>,
    pub(crate) network: Option<Sourced<bool>>,
    pub(crate) permission_profile: Option<ConfigSource>,
    pub(crate) user_source: SourceStatus,
    pub(crate) project_source: SourceStatus,
}

pub(crate) fn load(paths: &CodexConfigPaths) -> Result<ConfigSnapshot, CodexError> {
    let (user, user_source) =
        read_config(&paths.codex_home.join("config.toml"), ConfigSource::User)?;
    let (project, project_source) = match &paths.workspace_root {
        None => (None, SourceStatus::NotProvided),
        Some(_) if !paths.project_trusted => (None, SourceStatus::SkippedUntrusted),
        Some(root) => read_config(
            &root.join(".codex").join("config.toml"),
            ConfigSource::Project,
        )?,
    };

    Ok(ConfigSnapshot {
        sandbox: pick(
            project.as_ref().and_then(|c| c.sandbox),
            user.as_ref().and_then(|c| c.sandbox),
        ),
        approval: pick(
            project.as_ref().and_then(|c| c.approval),
            user.as_ref().and_then(|c| c.approval),
        ),
        network: pick(
            project.as_ref().and_then(|c| c.network),
            user.as_ref().and_then(|c| c.network),
        ),
        permission_profile: if project.as_ref().is_some_and(|c| c.permission_profile) {
            Some(ConfigSource::Project)
        } else if user.as_ref().is_some_and(|c| c.permission_profile) {
            Some(ConfigSource::User)
        } else {
            None
        },
        user_source,
        project_source,
    })
}

fn pick<T>(project: Option<T>, user: Option<T>) -> Option<Sourced<T>> {
    project
        .map(|value| Sourced {
            value,
            source: ConfigSource::Project,
        })
        .or_else(|| {
            user.map(|value| Sourced {
                value,
                source: ConfigSource::User,
            })
        })
}

fn read_config(
    path: &Path,
    source: ConfigSource,
) -> Result<(Option<FileConfig>, SourceStatus), CodexError> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok((None, SourceStatus::Missing));
        }
        Err(error) => {
            return Err(CodexError::Read {
                source,
                kind: error.kind(),
            });
        }
    };
    let table = contents
        .parse::<Table>()
        .map_err(|_| CodexError::InvalidToml { source })?;
    let config = parse_config(&table, source)?;
    Ok((Some(config), SourceStatus::Read))
}

fn parse_config(table: &Table, source: ConfigSource) -> Result<FileConfig, CodexError> {
    let sandbox = match table.get("sandbox_mode") {
        None => None,
        Some(Value::String(value)) => Some(match value.as_str() {
            "read-only" => SandboxMode::ReadOnly,
            "workspace-write" => SandboxMode::WorkspaceWrite,
            "danger-full-access" => SandboxMode::DangerFullAccess,
            _ => SandboxMode::Unknown,
        }),
        Some(_) => return Err(invalid(source, "sandbox_mode")),
    };
    let approval = match table.get("approval_policy") {
        None => None,
        Some(Value::String(value)) => Some(match value.as_str() {
            "on-request" => ApprovalMode::OnRequest,
            "never" => ApprovalMode::Never,
            "untrusted" => return Err(invalid(source, "approval_policy")),
            _ => ApprovalMode::Unknown,
        }),
        Some(Value::Table(value)) if value.contains_key("granular") => Some(ApprovalMode::Granular),
        Some(_) => return Err(invalid(source, "approval_policy")),
    };
    let network = match table.get("sandbox_workspace_write") {
        None => None,
        Some(Value::Table(value)) => match value.get("network_access") {
            None => None,
            Some(Value::Boolean(value)) => Some(*value),
            Some(_) => return Err(invalid(source, "sandbox_workspace_write.network_access")),
        },
        Some(_) => return Err(invalid(source, "sandbox_workspace_write")),
    };
    let permission_profile = table.contains_key("default_permissions")
        || table.contains_key("permissions")
        || (source == ConfigSource::User && table.contains_key("profile"));
    Ok(FileConfig {
        sandbox,
        approval,
        network,
        permission_profile,
    })
}

fn invalid(source: ConfigSource, setting: &'static str) -> CodexError {
    CodexError::InvalidSetting { source, setting }
}
