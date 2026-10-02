use std::fs;

use permbridge_codex::{
    CodexAdapter, CodexConfigPaths, CodexError, CodexSetting, ConfigSource, SettingObservation,
    SettingState, SourceStatus,
};
use permbridge_core::{
    compare, AgentAdapter, CanonicalPolicy, Capability, CapabilityObservation, ComparisonOutcome,
    ComparisonReason, Decision, DecisionRelation, EnforcementStrength, ExecutionRule, NetworkRule,
    PolicyScope,
};
use tempfile::TempDir;

fn desired() -> CanonicalPolicy {
    CanonicalPolicy::new(PolicyScope::Global)
}

fn adapter(root: &TempDir, trusted: bool) -> CodexAdapter {
    CodexAdapter::new(CodexConfigPaths::new(
        root.path().join("home"),
        Some(root.path().join("workspace")),
        trusted,
    ))
}

fn write_user(root: &TempDir, contents: &str) {
    let home = root.path().join("home");
    fs::create_dir_all(&home).unwrap();
    fs::write(home.join("config.toml"), contents).unwrap();
}

fn write_project(root: &TempDir, contents: &str) {
    let codex = root.path().join("workspace/.codex");
    fs::create_dir_all(&codex).unwrap();
    fs::write(codex.join("config.toml"), contents).unwrap();
}

fn observed(
    report: &permbridge_codex::CodexInspection,
    setting: CodexSetting,
) -> SettingObservation {
    *report
        .settings
        .iter()
        .find(|observation| observation.setting == setting)
        .unwrap()
}

#[test]
fn project_values_override_user_values_without_mutating_sources() {
    let root = tempfile::tempdir().unwrap();
    let user = include_str!("fixtures/user.toml");
    let project = include_str!("fixtures/project.toml");
    write_user(&root, user);
    write_project(&root, project);

    let adapter = adapter(&root, true);
    let report = adapter.inspect_report(&desired()).unwrap();

    assert_eq!(adapter.id(), "codex");
    assert_eq!(report.user_source, SourceStatus::Read);
    assert_eq!(report.project_source, SourceStatus::Read);
    assert_eq!(
        observed(&report, CodexSetting::SandboxMode),
        SettingObservation {
            setting: CodexSetting::SandboxMode,
            state: SettingState::Configured("workspace-write"),
            source: Some(ConfigSource::Project),
        }
    );
    assert_eq!(
        observed(&report, CodexSetting::ApprovalPolicy).source,
        Some(ConfigSource::User)
    );
    assert_eq!(
        observed(&report, CodexSetting::WorkspaceNetworkAccess),
        SettingObservation {
            setting: CodexSetting::WorkspaceNetworkAccess,
            state: SettingState::Configured("true"),
            source: Some(ConfigSource::Project),
        }
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::FilesystemWrite),
        Some(&CapabilityObservation::Known {
            decision: Decision::Allow,
            enforcement: EnforcementStrength::Declared,
        })
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::FilesystemOutsideWorkspace),
        Some(&CapabilityObservation::Ambiguous)
    );
    assert_eq!(
        report.posture.capabilities.get(&Capability::NetworkDefault),
        Some(&CapabilityObservation::Ambiguous)
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::ExecutionDefault),
        Some(&CapabilityObservation::Ambiguous)
    );
    assert_eq!(
        fs::read_to_string(root.path().join("home/config.toml")).unwrap(),
        user
    );
    assert_eq!(
        fs::read_to_string(root.path().join("workspace/.codex/config.toml")).unwrap(),
        project
    );
}

#[test]
fn missing_files_do_not_create_configuration_or_infer_defaults() {
    let root = tempfile::tempdir().unwrap();
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();

    assert_eq!(report.user_source, SourceStatus::Missing);
    assert_eq!(report.project_source, SourceStatus::Missing);
    assert!(report
        .settings
        .iter()
        .all(|entry| entry.state == SettingState::NotConfigured));
    assert!(report
        .posture
        .capabilities
        .values()
        .all(|value| *value == CapabilityObservation::NotConfigured));
    assert!(!root.path().join("home").exists());
    assert!(!root.path().join("workspace").exists());
}

#[test]
fn untrusted_project_file_is_not_read() {
    let root = tempfile::tempdir().unwrap();
    write_user(&root, include_str!("fixtures/user.toml"));
    write_project(&root, include_str!("fixtures/malformed.toml"));

    let report = adapter(&root, false).inspect_report(&desired()).unwrap();
    assert_eq!(report.project_source, SourceStatus::SkippedUntrusted);
    assert_eq!(
        observed(&report, CodexSetting::SandboxMode).state,
        SettingState::Configured("read-only")
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::FilesystemWrite),
        Some(&CapabilityObservation::Ambiguous)
    );
}

#[test]
fn malformed_and_invalid_supported_settings_fail_explicitly() {
    let root = tempfile::tempdir().unwrap();
    write_user(&root, include_str!("fixtures/malformed.toml"));
    assert!(matches!(
        adapter(&root, true).inspect_report(&desired()),
        Err(CodexError::InvalidToml {
            source: ConfigSource::User
        })
    ));

    write_user(&root, "sandbox_mode = 7\n");
    assert!(matches!(
        adapter(&root, true).inspect_report(&desired()),
        Err(CodexError::InvalidSetting {
            source: ConfigSource::User,
            setting: "sandbox_mode"
        })
    ));

    write_user(&root, "approval_policy = \"untrusted\"\n");
    assert!(matches!(
        adapter(&root, true).inspect_report(&desired()),
        Err(CodexError::InvalidSetting {
            source: ConfigSource::User,
            setting: "approval_policy"
        })
    ));
}

#[test]
fn unknown_values_and_permission_profiles_do_not_become_allow() {
    let root = tempfile::tempdir().unwrap();
    write_user(
        &root,
        "sandbox_mode = \"future-mode\"\napproval_policy = { granular = {} }\n",
    );
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(
        observed(&report, CodexSetting::SandboxMode).state,
        SettingState::Unknown
    );
    assert_eq!(
        observed(&report, CodexSetting::ApprovalPolicy).state,
        SettingState::Unsupported
    );
    assert_eq!(
        report.posture.capabilities.get(&Capability::FilesystemRead),
        Some(&CapabilityObservation::Ambiguous)
    );

    write_user(&root, include_str!("fixtures/profile.toml"));
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(
        observed(&report, CodexSetting::PermissionProfile).state,
        SettingState::Unsupported
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::FilesystemWrite),
        Some(&CapabilityObservation::Ambiguous)
    );
}

#[test]
fn selector_rules_are_explicitly_unsupported() {
    let root = tempfile::tempdir().unwrap();
    let mut policy = desired();
    policy.network.domain_rules.push(NetworkRule {
        domain: "example.test".into(),
        decision: Decision::Deny,
    });
    policy.execution.rules.push(ExecutionRule {
        command: "synthetic-command".into(),
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
            .get(&Capability::ExecutionCommand("synthetic-command".into())),
        Some(&CapabilityObservation::Unsupported)
    );
    assert_eq!(
        adapter(&root, true).inspect(&policy).unwrap(),
        report.posture
    );
}

#[test]
fn each_recognized_sandbox_mode_has_a_conservative_filesystem_mapping() {
    let root = tempfile::tempdir().unwrap();
    for (mode, write, outside) in [
        (
            "read-only",
            CapabilityObservation::Ambiguous,
            CapabilityObservation::Ambiguous,
        ),
        (
            "workspace-write",
            CapabilityObservation::Known {
                decision: Decision::Allow,
                enforcement: EnforcementStrength::Declared,
            },
            CapabilityObservation::Ambiguous,
        ),
        (
            "danger-full-access",
            CapabilityObservation::Known {
                decision: Decision::Allow,
                enforcement: EnforcementStrength::Declared,
            },
            CapabilityObservation::Known {
                decision: Decision::Allow,
                enforcement: EnforcementStrength::Declared,
            },
        ),
    ] {
        write_user(&root, &format!("sandbox_mode = \"{mode}\"\n"));
        let report = adapter(&root, true).inspect_report(&desired()).unwrap();
        assert_eq!(
            report.posture.capabilities.get(&Capability::FilesystemRead),
            Some(&CapabilityObservation::Known {
                decision: Decision::Allow,
                enforcement: EnforcementStrength::Declared,
            })
        );
        assert_eq!(
            report
                .posture
                .capabilities
                .get(&Capability::FilesystemWrite),
            Some(&write)
        );
        assert_eq!(
            report
                .posture
                .capabilities
                .get(&Capability::FilesystemOutsideWorkspace),
            Some(&outside)
        );
    }
}

#[test]
fn network_toggle_and_approval_policy_never_do_not_imply_global_decisions() {
    let root = tempfile::tempdir().unwrap();
    write_user(
        &root,
        "approval_policy = \"never\"\n[sandbox_workspace_write]\nnetwork_access = false\n",
    );
    let report = adapter(&root, true).inspect_report(&desired()).unwrap();
    assert_eq!(
        observed(&report, CodexSetting::ApprovalPolicy).state,
        SettingState::Configured("never")
    );
    assert_eq!(
        observed(&report, CodexSetting::WorkspaceNetworkAccess).state,
        SettingState::Configured("false")
    );
    assert_eq!(
        report.posture.capabilities.get(&Capability::NetworkDefault),
        Some(&CapabilityObservation::Ambiguous)
    );
    assert_eq!(
        report
            .posture
            .capabilities
            .get(&Capability::ExecutionDefault),
        Some(&CapabilityObservation::Ambiguous)
    );
}

#[test]
fn unreadable_source_and_invalid_network_type_have_explicit_errors() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("home/config.toml")).unwrap();
    assert!(matches!(
        adapter(&root, true).inspect_report(&desired()),
        Err(CodexError::Read {
            source: ConfigSource::User,
            ..
        })
    ));

    fs::remove_dir(root.path().join("home/config.toml")).unwrap();
    write_user(
        &root,
        "[sandbox_workspace_write]\nnetwork_access = \"yes\"\n",
    );
    assert!(matches!(
        adapter(&root, true).inspect_report(&desired()),
        Err(CodexError::InvalidSetting {
            source: ConfigSource::User,
            setting: "sandbox_workspace_write.network_access"
        })
    ));
}

#[test]
fn omitting_a_project_root_is_reported() {
    let root = tempfile::tempdir().unwrap();
    let paths = CodexConfigPaths::new(root.path().to_path_buf(), None, false);
    let report = CodexAdapter::new(paths).inspect_report(&desired()).unwrap();
    assert_eq!(report.project_source, SourceStatus::NotProvided);
}

#[test]
fn declared_codex_posture_feeds_comparator_without_a_passing_claim() {
    let root = tempfile::tempdir().unwrap();
    write_user(&root, include_str!("fixtures/user.toml"));
    write_project(&root, include_str!("fixtures/project.toml"));
    let desired = desired();
    let posture = adapter(&root, true).inspect(&desired).unwrap();
    let report = compare(&desired, &posture).unwrap();
    let write = &report.results[&Capability::FilesystemWrite];
    assert_eq!(
        write.decision_relation,
        Some(DecisionRelation::LessRestrictive)
    );
    assert_eq!(write.outcome, ComparisonOutcome::Ambiguous);
    assert_eq!(write.reason, ComparisonReason::DeclaredOnly);
    assert_eq!(
        report.results[&Capability::NetworkDefault].outcome,
        ComparisonOutcome::Ambiguous
    );
}
