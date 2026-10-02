# ADR 0004: Inspect Codex files conservatively

## Status

Accepted, 2026-10-02. The adapter API and supported mappings are experimental.

## Context

Codex has layered configuration, project trust, runtime overrides, managed
requirements, and multiple sandbox implementations. A TOML file alone cannot
prove the active boundary. The existing `AgentAdapter` contract lacked a
distinct result for an inspected but absent setting.

## Decision

Implement the first Codex adapter in `permbridge-codex`, outside Core. Accept
configuration roots and project trust from the caller. Read only user and
trusted root-project config files, use documented project-over-user precedence
for explicit supported keys, and expose selected source provenance. Do not
infer runtime defaults or claim sandbox enforcement from file text.

Add `CapabilityObservation::NotConfigured` to Core. Keep the generic adapter
trait unchanged; a Codex-specific `inspect_report` supplies evidence alongside
the trait's canonical posture. Parse TOML with `toml` rather than a partial
handwritten parser. Use `tempfile` only in tests for isolated config roots.

Map only documented, explicit legacy sandbox allowances to canonical
filesystem `Allow` with `Declared` strength. Keep read-only writes,
outside-workspace restrictions, network, and command execution ambiguous when
the broader canonical capability is not established. Treat permission profiles
and requested domain/command selectors as unsupported. Reject malformed
supported input without disclosing file contents or paths in errors.

## Consequences

The adapter supplies useful, testable configuration evidence but cannot claim
an effective runtime security posture. CLI flags, profiles, managed policy,
session changes, and OS sandbox behavior need later integration and behavior
tests. The [compatibility guide](../compatibility/codex.md) records the exact
mapping and OpenAI documentation used.
