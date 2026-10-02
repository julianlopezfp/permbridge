use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::{
    CanonicalPolicy, Capability, CapabilityObservation, Decision, EffectivePosture,
    EnforcementStrength,
};

/// Final relationship for one desired capability. Only ordered outcomes have
/// enough evidence for a decision comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonOutcome {
    /// Observed and desired decisions match.
    Equivalent,
    /// The observation is more restrictive.
    MoreRestrictive,
    /// The observation is less restrictive.
    LessRestrictive,
    /// The adapter cannot expose or express the capability.
    Unsupported,
    /// Inspected sources contain no explicit relevant setting.
    NotConfigured,
    /// Evidence is missing, uncertain, or declaration-only.
    Ambiguous,
}

impl ComparisonOutcome {
    /// Whether sufficiently evidenced behavior meets the desired restriction.
    /// `None` must not be treated as success.
    #[must_use]
    pub fn meets_or_exceeds_policy(self) -> Option<bool> {
        match self {
            Self::Equivalent | Self::MoreRestrictive => Some(true),
            Self::LessRestrictive => Some(false),
            Self::Unsupported | Self::NotConfigured | Self::Ambiguous => None,
        }
    }
}

/// Relationship of known decisions before evidence adequacy is considered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionRelation {
    /// Both decisions are equal.
    Equivalent,
    /// The observed decision is more restrictive.
    MoreRestrictive,
    /// The observed decision is less restrictive.
    LessRestrictive,
}

/// Machine-readable reason for the final outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonReason {
    /// The adapter reports an observed mechanism for this capability.
    ObservedMechanism,
    /// The mapped decision is only a stored declaration.
    DeclaredOnly,
    /// The adapter explicitly cannot map this capability.
    Unsupported,
    /// Native semantics could not be mapped confidently.
    AmbiguousObservation,
    /// Inspected sources had no explicit relevant setting.
    NotConfigured,
    /// The adapter omitted this desired capability.
    MissingObservation,
}

/// One desired decision and its optional agent observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityComparison {
    /// Desired permission decision.
    pub desired: Decision,
    /// Exact adapter observation, including its enforcement strength if known.
    pub observed: Option<CapabilityObservation>,
    /// Known decision relationship, independent of evidence strength.
    pub decision_relation: Option<DecisionRelation>,
    /// Final evidence-aware outcome.
    pub outcome: ComparisonOutcome,
    /// Stable explanation category.
    pub reason: ComparisonReason,
}

/// Deterministic results keyed by canonical capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonReport {
    /// Results for all desired baseline capabilities and selectors.
    pub results: BTreeMap<Capability, CapabilityComparison>,
    /// Agent observations with no desired rule; no conclusion is drawn for them.
    pub unrequested: BTreeMap<Capability, CapabilityObservation>,
}

/// The in-memory desired policy cannot yield one decision for a selector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonError {
    /// Repeated opaque selector with conflicting desired decisions.
    ConflictingDesiredRules(Capability),
}

impl fmt::Display for ComparisonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingDesiredRules(_) => {
                write!(f, "conflicting desired rules for one capability")
            }
        }
    }
}

impl Error for ComparisonError {}

/// Compare one desired in-memory policy with one adapter-produced posture.
///
/// Selector strings are exact, opaque keys. Equal duplicates coalesce and
/// conflicting duplicates fail; this function does not match action requests,
/// merge scopes, or establish runtime enforcement.
pub fn compare(
    desired: &CanonicalPolicy,
    posture: &EffectivePosture,
) -> Result<ComparisonReport, ComparisonError> {
    use Capability::{
        ExecutionCommand, ExecutionDefault, FilesystemOutsideWorkspace, FilesystemRead,
        FilesystemWrite, NetworkDefault, NetworkDomain,
    };

    let mut requested = BTreeMap::from([
        (FilesystemRead, desired.filesystem.read),
        (FilesystemWrite, desired.filesystem.write),
        (
            FilesystemOutsideWorkspace,
            desired.filesystem.outside_workspace,
        ),
        (NetworkDefault, desired.network.default_decision),
        (ExecutionDefault, desired.execution.default_decision),
    ]);
    for rule in &desired.network.domain_rules {
        insert_rule(
            &mut requested,
            NetworkDomain(rule.domain.clone()),
            rule.decision,
        )?;
    }
    for rule in &desired.execution.rules {
        insert_rule(
            &mut requested,
            ExecutionCommand(rule.command.clone()),
            rule.decision,
        )?;
    }

    let results = requested
        .iter()
        .map(|(capability, decision)| {
            (
                capability.clone(),
                compare_capability(*decision, posture.capabilities.get(capability)),
            )
        })
        .collect();
    let unrequested = posture
        .capabilities
        .iter()
        .filter(|(capability, _)| !requested.contains_key(*capability))
        .map(|(capability, observation)| (capability.clone(), observation.clone()))
        .collect();
    Ok(ComparisonReport {
        results,
        unrequested,
    })
}

fn insert_rule(
    requested: &mut BTreeMap<Capability, Decision>,
    capability: Capability,
    decision: Decision,
) -> Result<(), ComparisonError> {
    match requested.get(&capability) {
        Some(existing) if *existing != decision => {
            Err(ComparisonError::ConflictingDesiredRules(capability))
        }
        Some(_) => Ok(()),
        None => {
            requested.insert(capability, decision);
            Ok(())
        }
    }
}

fn compare_capability(
    desired: Decision,
    observed: Option<&CapabilityObservation>,
) -> CapabilityComparison {
    use CapabilityObservation::{Ambiguous, Known, NotConfigured, Unsupported};
    use ComparisonOutcome as Outcome;
    use ComparisonReason as Reason;

    let (decision_relation, outcome, reason) = match observed {
        None => (None, Outcome::Ambiguous, Reason::MissingObservation),
        Some(Unsupported) => (None, Outcome::Unsupported, Reason::Unsupported),
        Some(NotConfigured) => (None, Outcome::NotConfigured, Reason::NotConfigured),
        Some(Ambiguous) => (None, Outcome::Ambiguous, Reason::AmbiguousObservation),
        Some(Known {
            decision,
            enforcement,
        }) => {
            let relation = decision_relation(desired, *decision);
            match enforcement {
                EnforcementStrength::Declared => {
                    (Some(relation), Outcome::Ambiguous, Reason::DeclaredOnly)
                }
                EnforcementStrength::ToolMediated | EnforcementStrength::OsSandbox => {
                    let outcome = match relation {
                        DecisionRelation::Equivalent => Outcome::Equivalent,
                        DecisionRelation::MoreRestrictive => Outcome::MoreRestrictive,
                        DecisionRelation::LessRestrictive => Outcome::LessRestrictive,
                    };
                    (Some(relation), outcome, Reason::ObservedMechanism)
                }
            }
        }
    };
    CapabilityComparison {
        desired,
        observed: observed.cloned(),
        decision_relation,
        outcome,
        reason,
    }
}

fn decision_relation(desired: Decision, observed: Decision) -> DecisionRelation {
    use std::cmp::Ordering;

    match observed.cmp(&desired) {
        Ordering::Less => DecisionRelation::LessRestrictive,
        Ordering::Equal => DecisionRelation::Equivalent,
        Ordering::Greater => DecisionRelation::MoreRestrictive,
    }
}
