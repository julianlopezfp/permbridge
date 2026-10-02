//! Strict version-1 YAML input for the provider-agnostic policy model.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde_yaml_ng::{Mapping, Value};

use crate::{CanonicalPolicy, Decision, ExecutionRule, NetworkRule, PolicyScope};

/// A stable category and logical field path for a failed policy load.
/// Source text and selector values are deliberately excluded from errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyLoadError {
    /// The requested file could not be read as UTF-8 text.
    Io(io::ErrorKind),
    /// The input is not one well-formed YAML document.
    MalformedYaml,
    /// A required field is absent.
    MissingField(String),
    /// The schema version is a valid integer, but is unsupported.
    UnsupportedVersion(u64),
    /// A field or mapping key has the wrong YAML type.
    InvalidType(String),
    /// An unrecognized field was supplied.
    UnknownField(String),
    /// A decision is not exactly `allow`, `ask`, or `deny`.
    InvalidDecision(String),
    /// A scope is not one of the supported canonical scopes.
    InvalidScope(String),
    /// A selector is empty or only whitespace.
    InvalidSelector(String),
    /// A selector appears twice with the same decision.
    DuplicateRule(String),
    /// A selector appears twice with different decisions.
    ConflictingRule(String),
}

impl fmt::Display for PolicyLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(kind) => write!(f, "policy file read failed: {kind}"),
            Self::MalformedYaml => write!(f, "malformed policy YAML"),
            Self::MissingField(path) => write!(f, "missing required field: {path}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported policy schema version: {version}")
            }
            Self::InvalidType(path) => write!(f, "invalid YAML type at {path}"),
            Self::UnknownField(path) => write!(f, "unknown policy field: {path}"),
            Self::InvalidDecision(path) => write!(f, "invalid decision at {path}"),
            Self::InvalidScope(path) => write!(f, "invalid policy scope at {path}"),
            Self::InvalidSelector(path) => write!(f, "invalid selector at {path}"),
            Self::DuplicateRule(path) => write!(f, "duplicate selector at {path}"),
            Self::ConflictingRule(path) => write!(f, "conflicting selector at {path}"),
        }
    }
}

impl Error for PolicyLoadError {}

/// Parse and validate one version-1 YAML document into a canonical policy.
///
/// `version` and `scope` are required. Every omitted decision is `ASK`, and
/// omitted rule lists are empty. Unknown fields and duplicate selectors fail;
/// no invalid input is converted into a permissive policy.
pub fn load_policy_yaml(source: &str) -> Result<CanonicalPolicy, PolicyLoadError> {
    let value: Value =
        serde_yaml_ng::from_str(source).map_err(|_| PolicyLoadError::MalformedYaml)?;
    let root = mapping(&value, "policy")?;
    fields(
        root,
        "",
        &[
            "version",
            "scope",
            "default_decision",
            "filesystem",
            "network",
            "execution",
        ],
    )?;

    let version = root
        .get("version")
        .ok_or_else(|| PolicyLoadError::MissingField("version".into()))?
        .as_u64()
        .ok_or_else(|| PolicyLoadError::InvalidType("version".into()))?;
    if version != 1 {
        return Err(PolicyLoadError::UnsupportedVersion(version));
    }
    let scope = match required_string(root, "scope", "scope")? {
        "managed" => PolicyScope::Managed,
        "global" => PolicyScope::Global,
        "project" => PolicyScope::Project,
        "session" => PolicyScope::Session,
        _ => return Err(PolicyLoadError::InvalidScope("scope".into())),
    };

    let mut policy = CanonicalPolicy::new(scope);
    if let Some(value) = root.get("default_decision") {
        policy.default_decision = decision(value, "default_decision")?;
    }
    if let Some(value) = root.get("filesystem") {
        let section = mapping(value, "filesystem")?;
        fields(
            section,
            "filesystem",
            &["read", "write", "outside_workspace"],
        )?;
        if let Some(value) = section.get("read") {
            policy.filesystem.read = decision(value, "filesystem.read")?;
        }
        if let Some(value) = section.get("write") {
            policy.filesystem.write = decision(value, "filesystem.write")?;
        }
        if let Some(value) = section.get("outside_workspace") {
            policy.filesystem.outside_workspace = decision(value, "filesystem.outside_workspace")?;
        }
    }
    if let Some(value) = root.get("network") {
        let section = mapping(value, "network")?;
        fields(section, "network", &["default_decision", "domains"])?;
        if let Some(value) = section.get("default_decision") {
            policy.network.default_decision = decision(value, "network.default_decision")?;
        }
        if let Some(value) = section.get("domains") {
            let rules = sequence(value, "network.domains")?;
            let mut seen = BTreeMap::new();
            for (index, value) in rules.iter().enumerate() {
                let path = format!("network.domains[{index}]");
                let rule = mapping(value, &path)?;
                fields(rule, &path, &["domain", "decision"])?;
                let selector = selector(rule, "domain", &format!("{path}.domain"))?;
                let chosen = decision(
                    required(rule, "decision", &format!("{path}.decision"))?,
                    &format!("{path}.decision"),
                )?;
                check_duplicate(&mut seen, selector, chosen, &path)?;
                policy.network.domain_rules.push(NetworkRule {
                    domain: selector.to_owned(),
                    decision: chosen,
                });
            }
        }
    }
    if let Some(value) = root.get("execution") {
        let section = mapping(value, "execution")?;
        fields(section, "execution", &["default_decision", "rules"])?;
        if let Some(value) = section.get("default_decision") {
            policy.execution.default_decision = decision(value, "execution.default_decision")?;
        }
        if let Some(value) = section.get("rules") {
            let rules = sequence(value, "execution.rules")?;
            let mut seen = BTreeMap::new();
            for (index, value) in rules.iter().enumerate() {
                let path = format!("execution.rules[{index}]");
                let rule = mapping(value, &path)?;
                fields(rule, &path, &["command", "decision"])?;
                let selector = selector(rule, "command", &format!("{path}.command"))?;
                let chosen = decision(
                    required(rule, "decision", &format!("{path}.decision"))?,
                    &format!("{path}.decision"),
                )?;
                check_duplicate(&mut seen, selector, chosen, &path)?;
                policy.execution.rules.push(ExecutionRule {
                    command: selector.to_owned(),
                    decision: chosen,
                });
            }
        }
    }
    Ok(policy)
}

/// Read one UTF-8 YAML file without modifying it, then validate its policy.
/// This API performs no discovery, merging, provider inspection, or enforcement.
pub fn load_policy_file(path: &Path) -> Result<CanonicalPolicy, PolicyLoadError> {
    let source = fs::read_to_string(path).map_err(|error| PolicyLoadError::Io(error.kind()))?;
    load_policy_yaml(&source)
}

fn mapping<'a>(value: &'a Value, path: &str) -> Result<&'a Mapping, PolicyLoadError> {
    value
        .as_mapping()
        .ok_or_else(|| PolicyLoadError::InvalidType(path.into()))
}

fn sequence<'a>(value: &'a Value, path: &str) -> Result<&'a [Value], PolicyLoadError> {
    value
        .as_sequence()
        .map(Vec::as_slice)
        .ok_or_else(|| PolicyLoadError::InvalidType(path.into()))
}

fn fields(map: &Mapping, parent: &str, allowed: &[&str]) -> Result<(), PolicyLoadError> {
    for key in map.keys() {
        let name = key
            .as_str()
            .ok_or_else(|| PolicyLoadError::InvalidType(parent.into()))?;
        if !allowed.contains(&name) {
            let path = if parent.is_empty() {
                name.to_owned()
            } else {
                format!("{parent}.{name}")
            };
            return Err(PolicyLoadError::UnknownField(path));
        }
    }
    Ok(())
}

fn required<'a>(map: &'a Mapping, key: &str, path: &str) -> Result<&'a Value, PolicyLoadError> {
    map.get(key)
        .ok_or_else(|| PolicyLoadError::MissingField(path.into()))
}

fn required_string<'a>(
    map: &'a Mapping,
    key: &str,
    path: &str,
) -> Result<&'a str, PolicyLoadError> {
    required(map, key, path)?
        .as_str()
        .ok_or_else(|| PolicyLoadError::InvalidType(path.into()))
}

fn decision(value: &Value, path: &str) -> Result<Decision, PolicyLoadError> {
    match value.as_str() {
        Some("allow") => Ok(Decision::Allow),
        Some("ask") => Ok(Decision::Ask),
        Some("deny") => Ok(Decision::Deny),
        Some(_) => Err(PolicyLoadError::InvalidDecision(path.into())),
        None => Err(PolicyLoadError::InvalidType(path.into())),
    }
}

fn selector<'a>(map: &'a Mapping, key: &str, path: &str) -> Result<&'a str, PolicyLoadError> {
    let value = required_string(map, key, path)?;
    if value.trim().is_empty() {
        return Err(PolicyLoadError::InvalidSelector(path.into()));
    }
    Ok(value)
}

fn check_duplicate<'a>(
    seen: &mut BTreeMap<&'a str, Decision>,
    selector: &'a str,
    chosen: Decision,
    path: &str,
) -> Result<(), PolicyLoadError> {
    match seen.insert(selector, chosen) {
        Some(previous) if previous == chosen => Err(PolicyLoadError::DuplicateRule(path.into())),
        Some(_) => Err(PolicyLoadError::ConflictingRule(path.into())),
        None => Ok(()),
    }
}
