use std::collections::BTreeMap;
use std::error::Error;

use crate::{CanonicalPolicy, Decision};

/// A canonical capability that an adapter may observe. Selector variants are
/// opaque until their validation and matching semantics are specified.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    /// Reading files inside the workspace.
    FilesystemRead,
    /// Writing files inside the workspace.
    FilesystemWrite,
    /// Access outside the workspace, in addition to read or write behavior.
    FilesystemOutsideWorkspace,
    /// Network behavior when no specific domain rule applies.
    NetworkDefault,
    /// A domain-specific network rule with an opaque selector.
    NetworkDomain(String),
    /// Command behavior when no specific execution rule applies.
    ExecutionDefault,
    /// A command-specific execution rule with an opaque selector.
    ExecutionCommand(String),
}

/// The mechanism supporting an observed decision for one capability. These
/// categories describe evidence, not a universal guarantee of containment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnforcementStrength {
    /// The agent declares or stores a setting, but no enforcing gate is known.
    Declared,
    /// The agent mediates the action through a tool or approval gate.
    ToolMediated,
    /// An OS-level sandbox boundary enforces this specific capability.
    OsSandbox,
}

/// What an adapter can establish about one native capability. A known native
/// decision is not automatically equivalent to the desired decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityObservation {
    /// The adapter can describe an effective decision and its mechanism.
    Known {
        /// Native decision as interpreted by the adapter.
        decision: Decision,
        /// Mechanism observed for this specific capability.
        enforcement: EnforcementStrength,
    },
    /// The native agent cannot represent or expose the capability.
    Unsupported,
    /// The adapter cannot establish a reliable interpretation.
    Ambiguous,
}

/// Adapter observations keyed by canonical capability. Absence means the
/// capability was not inspected; it must not be interpreted as ALLOW or as an
/// equivalent mapping.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectivePosture {
    /// Observations produced for the requested policy capabilities.
    pub capabilities: BTreeMap<Capability, CapabilityObservation>,
}

/// Boundary for a concrete coding agent. An adapter inspects native settings
/// relevant to `desired` and reports observations, including unsupported or
/// ambiguous mappings. It does not decide the final comparison outcome.
pub trait AgentAdapter {
    /// Adapter-specific inspection failure.
    type Error: Error;

    /// Stable, machine-readable identifier for the agent integration.
    fn id(&self) -> &str;

    /// Inspect effective/native configuration for the desired capabilities.
    /// A successful result may still contain unsupported or ambiguous entries.
    fn inspect(&self, desired: &CanonicalPolicy) -> Result<EffectivePosture, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::io;

    use super::{
        AgentAdapter, Capability, CapabilityObservation, EffectivePosture, EnforcementStrength,
    };
    use crate::{CanonicalPolicy, Decision, PolicyScope};

    struct TestAdapter;
    struct FailingAdapter;

    impl AgentAdapter for TestAdapter {
        type Error = Infallible;

        fn id(&self) -> &str {
            "test-agent"
        }

        fn inspect(&self, desired: &CanonicalPolicy) -> Result<EffectivePosture, Self::Error> {
            let mut posture = EffectivePosture::default();
            posture.capabilities.insert(
                Capability::FilesystemRead,
                CapabilityObservation::Known {
                    decision: Decision::Allow,
                    enforcement: EnforcementStrength::Declared,
                },
            );
            posture.capabilities.insert(
                Capability::FilesystemWrite,
                CapabilityObservation::Known {
                    decision: desired.filesystem.write,
                    enforcement: EnforcementStrength::ToolMediated,
                },
            );
            posture.capabilities.insert(
                Capability::FilesystemOutsideWorkspace,
                CapabilityObservation::Known {
                    decision: Decision::Deny,
                    enforcement: EnforcementStrength::OsSandbox,
                },
            );
            posture.capabilities.insert(
                Capability::NetworkDefault,
                CapabilityObservation::Unsupported,
            );
            posture.capabilities.insert(
                Capability::ExecutionDefault,
                CapabilityObservation::Ambiguous,
            );
            Ok(posture)
        }
    }

    impl AgentAdapter for FailingAdapter {
        type Error = io::Error;

        fn id(&self) -> &str {
            "failing-test-agent"
        }

        fn inspect(&self, _: &CanonicalPolicy) -> Result<EffectivePosture, Self::Error> {
            Err(io::Error::other("native configuration unavailable"))
        }
    }

    #[test]
    fn adapter_preserves_observed_enforcement_and_mapping_limits() {
        let mut desired = CanonicalPolicy::new(PolicyScope::Global);
        desired.filesystem.write = Decision::Deny;
        let adapter = TestAdapter;
        let posture = adapter
            .inspect(&desired)
            .unwrap_or_else(|never| match never {});

        assert_eq!(adapter.id(), "test-agent");
        assert_eq!(
            posture.capabilities.get(&Capability::FilesystemRead),
            Some(&CapabilityObservation::Known {
                decision: Decision::Allow,
                enforcement: EnforcementStrength::Declared,
            })
        );
        assert_eq!(
            posture.capabilities.get(&Capability::FilesystemWrite),
            Some(&CapabilityObservation::Known {
                decision: Decision::Deny,
                enforcement: EnforcementStrength::ToolMediated,
            })
        );
        assert_eq!(
            posture
                .capabilities
                .get(&Capability::FilesystemOutsideWorkspace),
            Some(&CapabilityObservation::Known {
                decision: Decision::Deny,
                enforcement: EnforcementStrength::OsSandbox,
            })
        );
        assert_eq!(
            posture.capabilities.get(&Capability::NetworkDefault),
            Some(&CapabilityObservation::Unsupported)
        );
        assert_eq!(
            posture.capabilities.get(&Capability::ExecutionDefault),
            Some(&CapabilityObservation::Ambiguous)
        );
        assert_eq!(
            posture
                .capabilities
                .get(&Capability::NetworkDomain("example.com".into())),
            None
        );
    }

    #[test]
    fn inspection_failure_is_distinct_from_an_unsupported_capability() {
        let desired = CanonicalPolicy::new(PolicyScope::Session);
        let result = FailingAdapter.inspect(&desired);

        assert!(matches!(result, Err(error) if error.kind() == io::ErrorKind::Other));
    }
}
