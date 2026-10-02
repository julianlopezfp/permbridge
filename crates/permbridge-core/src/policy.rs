use crate::Decision;

/// The origin of a desired policy. Scope does not imply an ordering between
/// all sources; source precedence still needs a separate specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyScope {
    /// Policy controlled by an organization or other administrator.
    Managed,
    /// Policy owned by the local user across projects.
    Global,
    /// Policy associated with one repository or project.
    Project,
    /// Temporary policy for one agent session.
    Session,
}

/// Desired permissions for the capabilities currently represented by Core.
/// This in-memory model is experimental. YAML version 1 maps into it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPolicy {
    /// Origin of this policy. The Core does not merge scopes yet.
    pub scope: PolicyScope,
    /// Fallback for capabilities without a specific desired decision.
    pub default_decision: Decision,
    /// Filesystem permissions.
    pub filesystem: FilesystemPolicy,
    /// Network permissions.
    pub network: NetworkPolicy,
    /// Command execution permissions.
    pub execution: ExecutionPolicy,
}

impl CanonicalPolicy {
    /// Creates a policy whose decisions all require approval until specified.
    #[must_use]
    pub fn new(scope: PolicyScope) -> Self {
        Self {
            scope,
            default_decision: Decision::Ask,
            filesystem: FilesystemPolicy::default(),
            network: NetworkPolicy::default(),
            execution: ExecutionPolicy::default(),
        }
    }
}

/// Desired filesystem decisions. `outside_workspace` is an additional
/// boundary decision, not a replacement for the read or write decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilesystemPolicy {
    /// Reading files inside the workspace.
    pub read: Decision,
    /// Writing files inside the workspace.
    pub write: Decision,
    /// Access outside the workspace, in addition to read or write policy.
    pub outside_workspace: Decision,
}

impl Default for FilesystemPolicy {
    fn default() -> Self {
        Self {
            read: Decision::Ask,
            write: Decision::Ask,
            outside_workspace: Decision::Ask,
        }
    }
}

/// Desired network permissions. Domain rules are stored for future adapter
/// translation; Core does not match requests to domains yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkPolicy {
    /// Fallback network decision.
    pub default_decision: Decision,
    /// Agent-agnostic domain-specific decisions.
    pub domain_rules: Vec<NetworkRule>,
}

impl Default for NetworkPolicy {
    fn default() -> Self {
        Self {
            default_decision: Decision::Ask,
            domain_rules: Vec::new(),
        }
    }
}

/// A domain-specific desired decision. YAML selectors are nonempty opaque
/// strings; matching semantics are intentionally unspecified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkRule {
    /// Domain selector as supplied by the caller.
    pub domain: String,
    /// Desired decision for that selector.
    pub decision: Decision,
}

/// Desired command execution permissions. Rule matching is not implemented.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPolicy {
    /// Fallback decision for command execution.
    pub default_decision: Decision,
    /// Command-specific desired decisions.
    pub rules: Vec<ExecutionRule>,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            default_decision: Decision::Ask,
            rules: Vec::new(),
        }
    }
}

/// A command-specific desired decision. Command syntax and matching
/// semantics are intentionally unspecified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionRule {
    /// Command selector as supplied by the caller.
    pub command: String,
    /// Desired decision for that selector.
    pub decision: Decision,
}

#[cfg(test)]
mod tests {
    use super::{CanonicalPolicy, PolicyScope};
    use crate::Decision;

    #[test]
    fn new_policy_defaults_to_ask_across_dimensions() {
        let policy = CanonicalPolicy::new(PolicyScope::Project);

        assert_eq!(policy.scope, PolicyScope::Project);
        assert_eq!(policy.default_decision, Decision::Ask);
        assert_eq!(policy.filesystem.read, Decision::Ask);
        assert_eq!(policy.filesystem.write, Decision::Ask);
        assert_eq!(policy.filesystem.outside_workspace, Decision::Ask);
        assert_eq!(policy.network.default_decision, Decision::Ask);
        assert!(policy.network.domain_rules.is_empty());
        assert_eq!(policy.execution.default_decision, Decision::Ask);
        assert!(policy.execution.rules.is_empty());
    }
}
