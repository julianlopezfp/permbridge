#![warn(missing_docs)]

//! Agent-agnostic policy and observation types for PermBridge.
//!
//! This experimental API represents desired policy and adapter observations.
//! It does not parse YAML, inspect a real agent, or compare postures yet.

mod adapter;
mod comparison;
mod decision;
mod policy;

pub use adapter::{
    AgentAdapter, Capability, CapabilityObservation, EffectivePosture, EnforcementStrength,
};
pub use comparison::ComparisonOutcome;
pub use decision::Decision;
pub use policy::{
    CanonicalPolicy, ExecutionPolicy, ExecutionRule, FilesystemPolicy, NetworkPolicy, NetworkRule,
    PolicyScope,
};
