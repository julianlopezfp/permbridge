use std::fs;

use permbridge_core::{
    compare, load_policy_file, load_policy_yaml, CanonicalPolicy, Capability,
    CapabilityObservation, ComparisonOutcome, Decision, EffectivePosture, EnforcementStrength,
    PolicyLoadError, PolicyScope,
};

const MINIMAL: &str = "version: 1\nscope: global\n";

#[test]
fn minimal_policy_uses_ask_for_every_omitted_decision() {
    let loaded = load_policy_yaml(MINIMAL).unwrap();
    assert_eq!(loaded, CanonicalPolicy::new(PolicyScope::Global));

    let fallback_only =
        load_policy_yaml("version: 1\nscope: global\ndefault_decision: allow\n").unwrap();
    assert_eq!(fallback_only.default_decision, Decision::Allow);
    assert_eq!(fallback_only.filesystem.read, Decision::Ask);
    assert_eq!(fallback_only.network.default_decision, Decision::Ask);
    assert_eq!(fallback_only.execution.default_decision, Decision::Ask);
}

#[test]
fn full_policy_maps_every_dimension_and_all_decisions() {
    let yaml = r#"
version: 1
scope: project
default_decision: deny
filesystem:
  read: allow
  write: ask
  outside_workspace: deny
network:
  default_decision: ask
  domains:
    - domain: example.test
      decision: deny
    - domain: internal.test
      decision: allow
execution:
  default_decision: deny
  rules:
    - command: synthetic-build
      decision: ask
"#;
    let policy = load_policy_yaml(yaml).unwrap();
    assert_eq!(policy.scope, PolicyScope::Project);
    assert_eq!(policy.default_decision, Decision::Deny);
    assert_eq!(policy.filesystem.read, Decision::Allow);
    assert_eq!(policy.filesystem.write, Decision::Ask);
    assert_eq!(policy.filesystem.outside_workspace, Decision::Deny);
    assert_eq!(policy.network.default_decision, Decision::Ask);
    assert_eq!(policy.network.domain_rules.len(), 2);
    assert_eq!(policy.network.domain_rules[0].domain, "example.test");
    assert_eq!(policy.network.domain_rules[0].decision, Decision::Deny);
    assert_eq!(policy.network.domain_rules[1].decision, Decision::Allow);
    assert_eq!(policy.execution.default_decision, Decision::Deny);
    assert_eq!(policy.execution.rules[0].command, "synthetic-build");
    assert_eq!(policy.execution.rules[0].decision, Decision::Ask);
    assert_eq!(load_policy_yaml(yaml).unwrap(), policy);
}

#[test]
fn each_scope_is_supported() {
    for (name, expected) in [
        ("managed", PolicyScope::Managed),
        ("global", PolicyScope::Global),
        ("project", PolicyScope::Project),
        ("session", PolicyScope::Session),
    ] {
        assert_eq!(
            load_policy_yaml(&format!("version: 1\nscope: {name}\n"))
                .unwrap()
                .scope,
            expected
        );
    }
}

#[test]
fn syntax_version_and_structure_fail_explicitly() {
    for (yaml, expected) in [
        ("version: [", PolicyLoadError::MalformedYaml),
        (
            "version: 1\nversion: 2\nscope: global",
            PolicyLoadError::MalformedYaml,
        ),
        (
            "scope: global",
            PolicyLoadError::MissingField("version".into()),
        ),
        (
            "version: nope\nscope: global",
            PolicyLoadError::InvalidType("version".into()),
        ),
        (
            "version: 2\nscope: global",
            PolicyLoadError::UnsupportedVersion(2),
        ),
        ("version: 1", PolicyLoadError::MissingField("scope".into())),
        (
            "version: 1\nscope: LOCAL",
            PolicyLoadError::InvalidScope("scope".into()),
        ),
        ("[]", PolicyLoadError::InvalidType("policy".into())),
        (
            "version: 1\nscope: global\nfilesystem: []",
            PolicyLoadError::InvalidType("filesystem".into()),
        ),
        (
            "version: 1\nscope: global\nnetwork: {domains: {}}",
            PolicyLoadError::InvalidType("network.domains".into()),
        ),
    ] {
        assert_eq!(load_policy_yaml(yaml).unwrap_err(), expected);
    }
}

#[test]
fn unknown_fields_and_invalid_decisions_never_load() {
    for (yaml, expected) in [
        ("version: 1\nscope: global\ntargets: [codex]", PolicyLoadError::UnknownField("targets".into())),
        ("version: 1\nscope: global\nfilesystem: {writ: deny}", PolicyLoadError::UnknownField("filesystem.writ".into())),
        ("version: 1\nscope: global\nnetwork: {domains: [{domain: example.test, decision: deny, pattern: glob}]}", PolicyLoadError::UnknownField("network.domains[0].pattern".into())),
        ("version: 1\nscope: global\ndefault_decision: ALLOW", PolicyLoadError::InvalidDecision("default_decision".into())),
        ("version: 1\nscope: global\nfilesystem: {read: true}", PolicyLoadError::InvalidType("filesystem.read".into())),
        ("version: 1\nscope: global\nexecution: {rules: [{command: x}]}", PolicyLoadError::MissingField("execution.rules[0].decision".into())),
    ] {
        assert_eq!(load_policy_yaml(yaml).unwrap_err(), expected);
    }
}

#[test]
fn selectors_are_opaque_but_must_be_nonempty_and_unique() {
    let prefix = "version: 1\nscope: global\n";
    for (section, expected) in [
        ("network: {domains: [{domain: '  ', decision: deny}]}", PolicyLoadError::InvalidSelector("network.domains[0].domain".into())),
        ("execution: {rules: [{command: '', decision: deny}]}", PolicyLoadError::InvalidSelector("execution.rules[0].command".into())),
        ("network: {domains: [{domain: x.test, decision: deny}, {domain: x.test, decision: deny}]}", PolicyLoadError::DuplicateRule("network.domains[1]".into())),
        ("network: {domains: [{domain: x.test, decision: deny}, {domain: x.test, decision: allow}]}", PolicyLoadError::ConflictingRule("network.domains[1]".into())),
        ("execution: {rules: [{command: x, decision: ask}, {command: x, decision: deny}]}", PolicyLoadError::ConflictingRule("execution.rules[1]".into())),
    ] {
        assert_eq!(load_policy_yaml(&format!("{prefix}{section}")).unwrap_err(), expected);
    }
    let policy = load_policy_yaml(&format!(
        "{prefix}network: {{domains: [{{domain: '*.EXAMPLE.test', decision: deny}}]}}"
    ))
    .unwrap();
    assert_eq!(policy.network.domain_rules[0].domain, "*.EXAMPLE.test");
}

#[test]
fn errors_do_not_echo_source_values() {
    let secret = "PRIVATE_SELECTOR_123";
    let yaml = format!(
        "version: 1\nscope: global\nnetwork: {{domains: [{{domain: {secret}, decision: ALLOW}}]}}"
    );
    let error = load_policy_yaml(&yaml).unwrap_err();
    assert_eq!(
        error,
        PolicyLoadError::InvalidDecision("network.domains[0].decision".into())
    );
    assert!(!format!("{error:?} {error}").contains(secret));
}

#[test]
fn path_loader_reads_without_mutating_and_examples_are_valid() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("policy.yaml");
    fs::write(&path, MINIMAL).unwrap();
    assert_eq!(
        load_policy_file(&path).unwrap(),
        load_policy_yaml(MINIMAL).unwrap()
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), MINIMAL);
    assert!(matches!(
        load_policy_file(&directory.path().join("missing.yaml")),
        Err(PolicyLoadError::Io(_))
    ));

    let examples = concat!(env!("CARGO_MANIFEST_DIR"), "/../../policies/examples");
    for name in ["minimal.yaml", "global.yaml", "project.yaml"] {
        load_policy_file(&std::path::Path::new(examples).join(name)).unwrap();
    }
}

#[test]
fn loaded_policy_composes_with_existing_comparator() {
    let policy =
        load_policy_yaml("version: 1\nscope: project\nfilesystem: {write: deny}\n").unwrap();
    let mut posture = EffectivePosture::default();
    posture.capabilities.insert(
        Capability::FilesystemWrite,
        CapabilityObservation::Known {
            decision: Decision::Allow,
            enforcement: EnforcementStrength::ToolMediated,
        },
    );
    let report = compare(&policy, &posture).unwrap();
    assert_eq!(
        report.results[&Capability::FilesystemWrite].outcome,
        ComparisonOutcome::LessRestrictive
    );
}
