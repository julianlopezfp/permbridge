use permbridge_core::{
    compare, CanonicalPolicy, Capability, CapabilityObservation, ComparisonError,
    ComparisonOutcome, ComparisonReason, Decision, DecisionRelation, EffectivePosture,
    EnforcementStrength, ExecutionRule, NetworkRule, PolicyScope,
};

fn policy() -> CanonicalPolicy {
    CanonicalPolicy::new(PolicyScope::Global)
}

fn known(decision: Decision, enforcement: EnforcementStrength) -> CapabilityObservation {
    CapabilityObservation::Known {
        decision,
        enforcement,
    }
}

#[test]
fn allow_ask_and_deny_can_each_be_equivalent_with_observed_mechanism() {
    for decision in [Decision::Allow, Decision::Ask, Decision::Deny] {
        let mut desired = policy();
        desired.filesystem.read = decision;
        for enforcement in [
            EnforcementStrength::ToolMediated,
            EnforcementStrength::OsSandbox,
        ] {
            let mut posture = EffectivePosture::default();
            posture
                .capabilities
                .insert(Capability::FilesystemRead, known(decision, enforcement));
            let report = compare(&desired, &posture).unwrap();
            let result = &report.results[&Capability::FilesystemRead];
            assert_eq!(result.desired, decision);
            assert_eq!(result.observed, Some(known(decision, enforcement)));
            assert_eq!(result.decision_relation, Some(DecisionRelation::Equivalent));
            assert_eq!(result.outcome, ComparisonOutcome::Equivalent);
            assert_eq!(result.reason, ComparisonReason::ObservedMechanism);
            assert_eq!(result.outcome.meets_or_exceeds_policy(), Some(true));
        }
    }
}

#[test]
fn restriction_order_is_allow_ask_deny() {
    let mut desired = policy();
    desired.filesystem.write = Decision::Ask;
    let mut posture = EffectivePosture::default();
    for (decision, relation, outcome, passing) in [
        (
            Decision::Allow,
            DecisionRelation::LessRestrictive,
            ComparisonOutcome::LessRestrictive,
            Some(false),
        ),
        (
            Decision::Ask,
            DecisionRelation::Equivalent,
            ComparisonOutcome::Equivalent,
            Some(true),
        ),
        (
            Decision::Deny,
            DecisionRelation::MoreRestrictive,
            ComparisonOutcome::MoreRestrictive,
            Some(true),
        ),
    ] {
        posture.capabilities.insert(
            Capability::FilesystemWrite,
            known(decision, EnforcementStrength::ToolMediated),
        );
        let report = compare(&desired, &posture).unwrap();
        let result = &report.results[&Capability::FilesystemWrite];
        assert_eq!(result.decision_relation, Some(relation));
        assert_eq!(result.outcome, outcome);
        assert_eq!(result.outcome.meets_or_exceeds_policy(), passing);
    }
}

#[test]
fn declared_decisions_keep_their_relation_but_never_pass() {
    for (desired_decision, observed_decision, relation) in [
        (
            Decision::Allow,
            Decision::Allow,
            DecisionRelation::Equivalent,
        ),
        (
            Decision::Ask,
            Decision::Deny,
            DecisionRelation::MoreRestrictive,
        ),
        (
            Decision::Deny,
            Decision::Allow,
            DecisionRelation::LessRestrictive,
        ),
    ] {
        let mut desired = policy();
        desired.execution.default_decision = desired_decision;
        let mut posture = EffectivePosture::default();
        posture.capabilities.insert(
            Capability::ExecutionDefault,
            known(observed_decision, EnforcementStrength::Declared),
        );
        let result = &compare(&desired, &posture).unwrap().results[&Capability::ExecutionDefault];
        assert_eq!(result.decision_relation, Some(relation));
        assert_eq!(result.outcome, ComparisonOutcome::Ambiguous);
        assert_eq!(result.reason, ComparisonReason::DeclaredOnly);
        assert_eq!(result.outcome.meets_or_exceeds_policy(), None);
    }
}

#[test]
fn missing_unsupported_ambiguous_and_not_configured_are_distinct() {
    let desired = policy();
    let mut posture = EffectivePosture::default();
    for (observation, outcome, reason) in [
        (
            CapabilityObservation::Unsupported,
            ComparisonOutcome::Unsupported,
            ComparisonReason::Unsupported,
        ),
        (
            CapabilityObservation::Ambiguous,
            ComparisonOutcome::Ambiguous,
            ComparisonReason::AmbiguousObservation,
        ),
        (
            CapabilityObservation::NotConfigured,
            ComparisonOutcome::NotConfigured,
            ComparisonReason::NotConfigured,
        ),
    ] {
        posture
            .capabilities
            .insert(Capability::NetworkDefault, observation.clone());
        let result = &compare(&desired, &posture).unwrap().results[&Capability::NetworkDefault];
        assert_eq!(result.observed, Some(observation));
        assert_eq!(result.outcome, outcome);
        assert_eq!(result.reason, reason);
        assert_eq!(result.outcome.meets_or_exceeds_policy(), None);
    }
    posture.capabilities.remove(&Capability::NetworkDefault);
    let result = &compare(&desired, &posture).unwrap().results[&Capability::NetworkDefault];
    assert_eq!(result.observed, None);
    assert_eq!(result.outcome, ComparisonOutcome::Ambiguous);
    assert_eq!(result.reason, ComparisonReason::MissingObservation);
}

#[test]
fn mixed_results_have_deterministic_order_without_an_aggregate_pass() {
    let desired = policy();
    let mut posture = EffectivePosture::default();
    posture.capabilities.insert(
        Capability::FilesystemRead,
        known(Decision::Ask, EnforcementStrength::OsSandbox),
    );
    posture.capabilities.insert(
        Capability::FilesystemWrite,
        CapabilityObservation::Unsupported,
    );
    posture.capabilities.insert(
        Capability::NetworkDefault,
        CapabilityObservation::NotConfigured,
    );
    let report = compare(&desired, &posture).unwrap();
    assert_eq!(
        report.results.keys().cloned().collect::<Vec<_>>(),
        vec![
            Capability::FilesystemRead,
            Capability::FilesystemWrite,
            Capability::FilesystemOutsideWorkspace,
            Capability::NetworkDefault,
            Capability::ExecutionDefault,
        ]
    );
    assert_eq!(
        report.results[&Capability::FilesystemRead].outcome,
        ComparisonOutcome::Equivalent
    );
    assert_eq!(
        report.results[&Capability::FilesystemWrite].outcome,
        ComparisonOutcome::Unsupported
    );
    assert_eq!(
        report.results[&Capability::NetworkDefault].outcome,
        ComparisonOutcome::NotConfigured
    );
    assert_eq!(
        report.results[&Capability::ExecutionDefault].reason,
        ComparisonReason::MissingObservation
    );
}

#[test]
fn unrequested_observations_are_preserved_but_not_compared() {
    let desired = policy();
    let extra = Capability::NetworkDomain("unexpected.test".into());
    let mut posture = EffectivePosture::default();
    posture
        .capabilities
        .insert(extra.clone(), CapabilityObservation::Ambiguous);
    let report = compare(&desired, &posture).unwrap();
    assert!(!report.results.contains_key(&extra));
    assert_eq!(
        report.unrequested.get(&extra),
        Some(&CapabilityObservation::Ambiguous)
    );

    // The posture's BTreeMap contains one value per key, so duplicates cannot reach compare.
    posture
        .capabilities
        .insert(Capability::FilesystemRead, CapabilityObservation::Ambiguous);
    posture.capabilities.insert(
        Capability::FilesystemRead,
        CapabilityObservation::Unsupported,
    );
    assert_eq!(posture.capabilities.len(), 2);
    assert_eq!(
        compare(&desired, &posture).unwrap().results[&Capability::FilesystemRead].outcome,
        ComparisonOutcome::Unsupported
    );
}

#[test]
fn exact_duplicate_selectors_coalesce_and_conflicts_fail() {
    let mut desired = policy();
    desired.network.domain_rules.push(NetworkRule {
        domain: "example.test".into(),
        decision: Decision::Deny,
    });
    desired.network.domain_rules.push(NetworkRule {
        domain: "example.test".into(),
        decision: Decision::Deny,
    });
    desired.execution.rules.push(ExecutionRule {
        command: "synthetic-command".into(),
        decision: Decision::Ask,
    });
    let posture = EffectivePosture::default();
    let report = compare(&desired, &posture).unwrap();
    assert_eq!(report.results.len(), 7);
    assert_eq!(
        report.results[&Capability::NetworkDomain("example.test".into())].desired,
        Decision::Deny
    );

    desired.network.domain_rules[1].decision = Decision::Allow;
    assert_eq!(
        compare(&desired, &posture),
        Err(ComparisonError::ConflictingDesiredRules(
            Capability::NetworkDomain("example.test".into())
        ))
    );
}

#[test]
fn selectors_are_exact_keys_and_top_level_fallback_adds_no_rule() {
    let mut desired = policy();
    desired.default_decision = Decision::Deny;
    desired.network.domain_rules.push(NetworkRule {
        domain: "example.test".into(),
        decision: Decision::Allow,
    });
    let mut posture = EffectivePosture::default();
    posture.capabilities.insert(
        Capability::NetworkDomain("example.test".into()),
        known(Decision::Ask, EnforcementStrength::ToolMediated),
    );
    posture.capabilities.insert(
        Capability::NetworkDomain("other.test".into()),
        CapabilityObservation::Ambiguous,
    );
    let report = compare(&desired, &posture).unwrap();
    assert_eq!(report.results.len(), 6);
    assert_eq!(
        report.results[&Capability::FilesystemRead].desired,
        Decision::Ask
    );
    assert_eq!(
        report.results[&Capability::NetworkDomain("example.test".into())].outcome,
        ComparisonOutcome::MoreRestrictive
    );
    assert!(report
        .unrequested
        .contains_key(&Capability::NetworkDomain("other.test".into())));
}

#[test]
fn only_evidenced_ordered_outcomes_have_a_boolean_result() {
    assert_eq!(
        ComparisonOutcome::Equivalent.meets_or_exceeds_policy(),
        Some(true)
    );
    assert_eq!(
        ComparisonOutcome::MoreRestrictive.meets_or_exceeds_policy(),
        Some(true)
    );
    assert_eq!(
        ComparisonOutcome::LessRestrictive.meets_or_exceeds_policy(),
        Some(false)
    );
    for outcome in [
        ComparisonOutcome::Unsupported,
        ComparisonOutcome::NotConfigured,
        ComparisonOutcome::Ambiguous,
    ] {
        assert_eq!(outcome.meets_or_exceeds_policy(), None);
    }
}
