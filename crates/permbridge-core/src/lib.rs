#![warn(missing_docs)]

//! Agent-agnostic policy and observation types for PermBridge.
//!
//! This experimental API represents desired policy and adapter observations.
//! YAML policies can be loaded into the canonical model. Loading does not
//! inspect an agent or enforce a policy.

mod adapter;
mod comparison;
mod decision;
mod loading;
mod policy;

pub use adapter::{
    AgentAdapter, Capability, CapabilityObservation, EffectivePosture, EnforcementStrength,
};
pub use comparison::{
    compare, CapabilityComparison, ComparisonError, ComparisonOutcome, ComparisonReason,
    ComparisonReport, DecisionRelation,
};
pub use decision::Decision;
pub use loading::{load_policy_file, load_policy_yaml, PolicyLoadError};
pub use policy::{
    CanonicalPolicy, ExecutionPolicy, ExecutionRule, FilesystemPolicy, NetworkPolicy, NetworkRule,
    PolicyScope,
};
