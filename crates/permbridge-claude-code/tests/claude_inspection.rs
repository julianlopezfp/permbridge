use std::fs;

use permbridge_claude_code::{
    ClaudeCodeAdapter, ClaudeConfigPaths, ClaudeError, ClaudeInspection, ClaudeSetting,
    ConfigSource, SettingState, SourceStatus,
};
use permbridge_core::{
    compare, AgentAdapter, CanonicalPolicy, Capability, CapabilityObservation, ComparisonOutcome,
    ComparisonReason, Decision, ExecutionRule, NetworkRule, PolicyScope,
};
use tempfile::TempDir;

fn desired() -> CanonicalPolicy {
    CanonicalPolicy::new(PolicyScope::Global)
}

fn adapter(root: &TempDir, trusted: bool) -> ClaudeCodeAdapter {
    ClaudeCodeAdapter::new(ClaudeConfigPaths::new(
        root.path().join("home"),
        Some(root.path().join("workspace")),
        trusted,
    ))
}

fn write(root: &TempDir, source: ConfigSource, contents: &str) {
    let path = match source {
        ConfigSource::User => root.path().join("home/settings.json"),
        ConfigSource::Project => root.path().join("workspace/.claude/settings.json"),
        ConfigSource::Local => root.path().join("workspace/.claude/settings.local.json"),
    };
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}

fn setting(
    report: &ClaudeInspection,
    name: ClaudeSetting,
) -> &permbridge_claude_code::SettingObservation {
    report
        .settings
        .iter()
        .find(|value| value.setting == name)
        .unwrap()
}

#[test]
fn scalar_precedence_and_rule_merging_preserve_provenance_without_mutation() {
    let root = tempfile::tempdir().unwrap();
    let sources = [
        (ConfigSource::User, include_str!("fixtures/user.json")),
        (ConfigSource::Project, include_str!("fixtures/project.json")),
        (ConfigSource::Local, include_str!("fixtures/local.json")),
    ];
    for (source, contents) in sources {
        write(&root, source, contents);
    }
    let adapter = adapter(&root, true);
    let report = adapter.inspect_report(&desired()).unwrap();
    assert_eq!(adapter.id(), "claude-code");
    assert_eq!(report.user_source, SourceStatus::Read);
    assert_eq!(report.project_source, SourceStatus::Read);
    assert_eq!(report.local_source, SourceStatus::Read);
    assert_eq!(
        setting(&report, ClaudeSetting::PermissionMode).state,
        SettingState::Configured("plan")
    );
    assert_eq!(
        setting(&report, ClaudeSetting::PermissionMode).sources,
        [ConfigSource::Local]
    );
    assert_eq!(
        setting(&report, ClaudeSetting::SandboxEnabled).state,
        SettingState::Configured("true")
    );
    assert_eq!(
        setting(&report, ClaudeSetting::SandboxEnabled).sources,
        [ConfigSource::Project]
    );
    assert_eq!(
        setting(&report, ClaudeSetting::AllowRules).sources,
        [
            ConfigSource::User,
            ConfigSource::Project,
            ConfigSource::Local
        ]
    );
    assert_eq!(
        setting(&report, ClaudeSetting::AskRules).sources,
        [ConfigSource::Project]
    );
    assert_eq!(
        setting(&report, ClaudeSetting::DenyRules).sources,
        [ConfigSource::User]
    );
    assert_eq!(
        setting(&report, ClaudeSetting::SandboxAllowUnsandboxedCommands).sources,
        [ConfigSource::Local]
    );
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::Ambiguous));
    for (source, contents) in sources {
        let path = match source {
            ConfigSource::User => root.path().join("home/settings.json"),
            ConfigSource::Project => root.path().join("workspace/.claude/settings.json"),
            ConfigSource::Local => root.path().join("workspace/.claude/settings.local.json"),
        };
        assert_eq!(fs::read_to_string(path).unwrap(), contents);
    }
}

#[test]
fn missing_files_do_not_create_configuration_or_infer_defaults() {
    let root = tempfile::tempdir().unwrap();
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(report.user_source, SourceStatus::Missing);
    assert_eq!(report.project_source, SourceStatus::Missing);
    assert_eq!(report.local_source, SourceStatus::Missing);
    assert!(report
        .settings
        .iter()
        .all(|value| value.state == SettingState::NotConfigured));
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::NotConfigured));
    assert!(!root.path().join("home").exists());
    assert!(!root.path().join("workspace").exists());
}

#[test]
fn untrusted_project_files_are_skipped() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        include_str!("fixtures/user.json"),
    );
    write(
        &root,
        ConfigSource::Project,
        include_str!("fixtures/malformed.json"),
    );
    write(
        &root,
        ConfigSource::Local,
        include_str!("fixtures/malformed.json"),
    );
    let report = adapter(&root, false).inspect_report(&desired()).unwrap();
    assert_eq!(report.project_source, SourceStatus::SkippedUntrusted);
    assert_eq!(report.local_source, SourceStatus::SkippedUntrusted);
    assert_eq!(
        setting(&report, ClaudeSetting::PermissionMode).sources,
        [ConfigSource::User]
    );
}

#[test]
fn malformed_and_invalid_supported_values_fail_without_exposing_contents() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        include_str!("fixtures/malformed.json"),
    );
    let error = adapter(&root, true).inspect_report(&desired()).unwrap_err();
    assert!(matches!(
        error,
        ClaudeError::InvalidJson {
            source: ConfigSource::User
        }
    ));
    assert!(!error.to_string().contains("permissions"));

    for (text, key) in [
        (
            r#"{"permissions":{"defaultMode":7}}"#,
            "permissions.defaultMode",
        ),
        (r#"{"permissions":{"allow":["Read",9]}}"#, "allow"),
        (r#"{"sandbox":{"enabled":"yes"}}"#, "enabled"),
        (r#"{"hooks":[]}"#, "hooks"),
    ] {
        write(&root, ConfigSource::User, text);
        assert!(matches!(adapter(&root, true).inspect_report(&desired()),
            Err(ClaudeError::InvalidSetting { source: ConfigSource::User, setting }) if setting == key));
    }
}

#[test]
fn unknown_mode_is_visible_without_becoming_a_decision() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        r#"{"permissions":{"defaultMode":"future-mode"}}"#,
    );
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(
        setting(&report, ClaudeSetting::PermissionMode).state,
        SettingState::Unknown
    );
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::Ambiguous));
}

#[test]
fn every_documented_mode_remains_ambiguous_for_broad_capabilities() {
    let root = tempfile::tempdir().unwrap();
    for mode in [
        "default",
        "manual",
        "acceptEdits",
        "plan",
        "auto",
        "dontAsk",
        "bypassPermissions",
    ] {
        write(
            &root,
            ConfigSource::User,
            &format!(r#"{{"permissions":{{"defaultMode":"{mode}"}}}}"#),
        );
        let report = adapter(&root, true).inspect_report(&desired()).unwrap();
        assert!(matches!(
            setting(&report, ClaudeSetting::PermissionMode).state,
            SettingState::Configured(_)
        ));
        assert!(report
            .posture
            .capabilities
            .values()
            .all(|value| *value == CapabilityObservation::Ambiguous));
    }
}

#[test]
fn sandbox_declarations_do_not_claim_os_enforcement_or_global_network_control() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        r#"{"sandbox":{"enabled":true,"failIfUnavailable":true,"allowUnsandboxedCommands":false,"filesystem":{},"network":{}},"hooks":{}}"#,
    );
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(
        setting(&report, ClaudeSetting::SandboxFilesystem).state,
        SettingState::Unsupported
    );
    assert_eq!(
        setting(&report, ClaudeSetting::SandboxNetwork).state,
        SettingState::Unsupported
    );
    assert_eq!(
        setting(&report, ClaudeSetting::Hooks).state,
        SettingState::Unsupported
    );
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::Ambiguous));
}

#[test]
fn requested_selectors_are_unsupported_even_when_native_rules_exist() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        include_str!("fixtures/user.json"),
    );
    let mut policy = desired();
    policy.network.domain_rules.push(NetworkRule {
        domain: "example.test".into(),
        decision: Decision::Deny,
    });
    policy.execution.rules.push(ExecutionRule {
        command: "synthetic-tool".into(),
        decision: Decision::Ask,
    });
    let report = adapter(&root, true).inspect_report(&policy).unwrap();
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::NetworkDomain("example.test".into())),
        Some(&CapabilityObservation::Unsupported)
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::ExecutionCommand("synthetic-tool".into())),
        Some(&CapabilityObservation::Unsupported)
    );
    assert_eq!(
        adapter(&root, true).inspect(&policy).unwrap(),
        report.posture
    );
}

#[test]
fn unreadable_source_has_a_sanitized_error() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("home/settings.json")).unwrap();
    let error = adapter(&root, true).inspect_report(&desired()).unwrap_err();
    assert!(matches!(
        error,
        ClaudeError::Read {
            source: ConfigSource::User,
            ..
        }
    ));
    assert!(!error
        .to_string()
        .contains(&root.path().to_string_lossy().to_string()));
}

#[test]
fn omitted_workspace_is_reported() {
    let root = tempfile::tempdir().unwrap();
    let adapter = ClaudeCodeAdapter::new(ClaudeConfigPaths::new(root.path().into(), None, false));
    let report = adapter.inspect_report(&desired()).unwrap();
    assert_eq!(report.project_source, SourceStatus::NotProvided);
    assert_eq!(report.local_source, SourceStatus::NotProvided);
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::Ambiguous));
}

#[test]
fn claude_posture_feeds_comparator_without_inventing_decisions() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root,
        ConfigSource::User,
        include_str!("fixtures/user.json"),
    );
    write(
        &root,
        ConfigSource::Project,
        include_str!("fixtures/project.json"),
    );
    write(
        &root,
        ConfigSource::Local,
        include_str!("fixtures/local.json"),
    );
    let desired = desired();
    let posture = adapter(&root, true).inspect(&desired).unwrap();
    let report = compare(&desired, &posture).unwrap();
    let write = &report.results[&Capability::FilesystemWrite];
    assert_eq!(write.observed, Some(CapabilityObservation::Ambiguous));
    assert_eq!(write.decision_relation, None);
    assert_eq!(write.outcome, ComparisonOutcome::Ambiguous);
    assert_eq!(write.reason, ComparisonReason::AmbiguousObservation);
}
