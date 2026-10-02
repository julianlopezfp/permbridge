use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Map, Value};

use crate::{ClaudeConfigPaths, ClaudeError, ConfigSource, SourceStatus};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PermissionMode {
    Default,
    AcceptEdits,
    Plan,
    Auto,
    DontAsk,
    BypassPermissions,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Sourced<T> {
    pub(crate) value: T,
    pub(crate) source: ConfigSource,
}

#[derive(Default)]
struct FileConfig {
    mode: Option<PermissionMode>,
    allow: bool,
    ask: bool,
    deny: bool,
    additional_directories: bool,
    sandbox_enabled: Option<bool>,
    fail_if_unavailable: Option<bool>,
    allow_unsandboxed_commands: Option<bool>,
    sandbox_filesystem: bool,
    sandbox_network: bool,
    hooks: bool,
}

pub(crate) struct ConfigSnapshot {
    pub(crate) mode: Option<Sourced<PermissionMode>>,
    pub(crate) allow: Vec<ConfigSource>,
    pub(crate) ask: Vec<ConfigSource>,
    pub(crate) deny: Vec<ConfigSource>,
    pub(crate) additional_directories: Vec<ConfigSource>,
    pub(crate) sandbox_enabled: Option<Sourced<bool>>,
    pub(crate) fail_if_unavailable: Option<Sourced<bool>>,
    pub(crate) allow_unsandboxed_commands: Option<Sourced<bool>>,
    pub(crate) sandbox_filesystem: Vec<ConfigSource>,
    pub(crate) sandbox_network: Vec<ConfigSource>,
    pub(crate) hooks: Vec<ConfigSource>,
    pub(crate) user_source: SourceStatus,
    pub(crate) project_source: SourceStatus,
    pub(crate) local_source: SourceStatus,
}

pub(crate) fn load(paths: &ClaudeConfigPaths) -> Result<ConfigSnapshot, ClaudeError> {
    let (user, user_source) =
        read_config(&paths.config_dir.join("settings.json"), ConfigSource::User)?;
    let (project, project_source, local, local_source) = match &paths.workspace_root {
        None => (
            None,
            SourceStatus::NotProvided,
            None,
            SourceStatus::NotProvided,
        ),
        Some(_) if !paths.project_trusted => (
            None,
            SourceStatus::SkippedUntrusted,
            None,
            SourceStatus::SkippedUntrusted,
        ),
        Some(root) => {
            let directory = root.join(".claude");
            let (project, project_source) =
                read_config(&directory.join("settings.json"), ConfigSource::Project)?;
            let (local, local_source) =
                read_config(&directory.join("settings.local.json"), ConfigSource::Local)?;
            (project, project_source, local, local_source)
        }
    };

    let layers = [
        (ConfigSource::User, user.as_ref()),
        (ConfigSource::Project, project.as_ref()),
        (ConfigSource::Local, local.as_ref()),
    ];
    Ok(ConfigSnapshot {
        mode: select(&layers, |file| file.mode),
        allow: sources(&layers, |file| file.allow),
        ask: sources(&layers, |file| file.ask),
        deny: sources(&layers, |file| file.deny),
        additional_directories: sources(&layers, |file| file.additional_directories),
        sandbox_enabled: select(&layers, |file| file.sandbox_enabled),
        fail_if_unavailable: select(&layers, |file| file.fail_if_unavailable),
        allow_unsandboxed_commands: select(&layers, |file| file.allow_unsandboxed_commands),
        sandbox_filesystem: sources(&layers, |file| file.sandbox_filesystem),
        sandbox_network: sources(&layers, |file| file.sandbox_network),
        hooks: sources(&layers, |file| file.hooks),
        user_source,
        project_source,
        local_source,
    })
}

fn select<T: Copy>(
    layers: &[(ConfigSource, Option<&FileConfig>); 3],
    get: impl Fn(&FileConfig) -> Option<T>,
) -> Option<Sourced<T>> {
    layers.iter().rev().find_map(|(source, file)| {
        file.and_then(&get).map(|value| Sourced {
            value,
            source: *source,
        })
    })
}

fn sources(
    layers: &[(ConfigSource, Option<&FileConfig>); 3],
    get: impl Fn(&FileConfig) -> bool,
) -> Vec<ConfigSource> {
    layers
        .iter()
        .filter_map(|(source, file)| file.filter(|file| get(file)).map(|_| *source))
        .collect()
}

fn read_config(
    path: &Path,
    source: ConfigSource,
) -> Result<(Option<FileConfig>, SourceStatus), ClaudeError> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok((None, SourceStatus::Missing))
        }
        Err(error) => {
            return Err(ClaudeError::Read {
                source,
                kind: error.kind(),
            })
        }
    };
    let value: Value =
        serde_json::from_str(&contents).map_err(|_| ClaudeError::InvalidJson { source })?;
    let object = value
        .as_object()
        .ok_or_else(|| invalid(source, "settings root"))?;
    Ok((Some(parse_config(object, source)?), SourceStatus::Read))
}

fn parse_config(
    object: &Map<String, Value>,
    source: ConfigSource,
) -> Result<FileConfig, ClaudeError> {
    let permissions = object
        .get("permissions")
        .map(|value| as_object(value, source, "permissions"))
        .transpose()?;
    let sandbox = object
        .get("sandbox")
        .map(|value| as_object(value, source, "sandbox"))
        .transpose()?;
    let mode = permissions
        .and_then(|value| value.get("defaultMode"))
        .map(|value| match value.as_str() {
            Some("default" | "manual") => Ok(PermissionMode::Default),
            Some("acceptEdits") => Ok(PermissionMode::AcceptEdits),
            Some("plan") => Ok(PermissionMode::Plan),
            Some("auto") => Ok(PermissionMode::Auto),
            Some("dontAsk") => Ok(PermissionMode::DontAsk),
            Some("bypassPermissions") => Ok(PermissionMode::BypassPermissions),
            Some(_) => Ok(PermissionMode::Unknown),
            None => Err(invalid(source, "permissions.defaultMode")),
        })
        .transpose()?;
    let rules = |name: &'static str| -> Result<bool, ClaudeError> {
        permissions
            .and_then(|value| value.get(name))
            .map(|value| string_array(value, source, name).map(|_| true))
            .transpose()
            .map(|value| value.unwrap_or(false))
    };
    let sandbox_bool = |name: &'static str| -> Result<Option<bool>, ClaudeError> {
        sandbox
            .and_then(|value| value.get(name))
            .map(|value| value.as_bool().ok_or_else(|| invalid(source, name)))
            .transpose()
    };
    let sandbox_object = |name: &'static str| -> Result<bool, ClaudeError> {
        sandbox
            .and_then(|value| value.get(name))
            .map(|value| as_object(value, source, name).map(|_| true))
            .transpose()
            .map(|value| value.unwrap_or(false))
    };
    let additional_directories = permissions
        .and_then(|value| value.get("additionalDirectories"))
        .map(|value| string_array(value, source, "permissions.additionalDirectories").map(|_| true))
        .transpose()?
        .unwrap_or(false);
    let hooks = object
        .get("hooks")
        .map(|value| as_object(value, source, "hooks").map(|_| true))
        .transpose()?
        .unwrap_or(false);
    Ok(FileConfig {
        mode,
        allow: rules("allow")?,
        ask: rules("ask")?,
        deny: rules("deny")?,
        additional_directories,
        sandbox_enabled: sandbox_bool("enabled")?,
        fail_if_unavailable: sandbox_bool("failIfUnavailable")?,
        allow_unsandboxed_commands: sandbox_bool("allowUnsandboxedCommands")?,
        sandbox_filesystem: sandbox_object("filesystem")?,
        sandbox_network: sandbox_object("network")?,
        hooks,
    })
}

fn as_object<'a>(
    value: &'a Value,
    source: ConfigSource,
    key: &'static str,
) -> Result<&'a Map<String, Value>, ClaudeError> {
    value.as_object().ok_or_else(|| invalid(source, key))
}

fn string_array(value: &Value, source: ConfigSource, key: &'static str) -> Result<(), ClaudeError> {
    match value.as_array() {
        Some(values) if values.iter().all(Value::is_string) => Ok(()),
        _ => Err(invalid(source, key)),
    }
}

fn invalid(source: ConfigSource, setting: &'static str) -> ClaudeError {
    ClaudeError::InvalidSetting { source, setting }
}
