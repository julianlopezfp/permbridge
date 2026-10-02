# ADR 0002: Keep agent integrations outside domain types

## Status

Accepted, 2026-10-02. The adapter trait is experimental.

## Context

Native permission files, approval flows, and evidence differ by agent and can
change independently. Placing provider-specific fields or SDKs in the
canonical policy model would couple every domain change to an integration and
could make native guarantees appear universal.

## Decision

Core defines a small `AgentAdapter` contract and shared observation types.
Concrete agents will implement that boundary outside the domain model.
Adapters identify themselves, inspect relevant native configuration, and
report known, not configured, unsupported, or ambiguous observations. The
Core comparator, not the adapter, decides policy equivalence under stated
evidence limits.

## Consequences

Core remains independent of provider SDKs and UI concerns. Each adapter must
document evidence limits and be tested against native behavior before support
is claimed. The separation adds an explicit mapping step, but it prevents an
integration from silently promoting a partial observation into a security
guarantee. The Codex adapter uses `NotConfigured` when inspected files contain
no explicit relevant setting. It does not infer a runtime default. The adapter
exists as an inspection-only integration; its limits are documented in the
[Codex compatibility guide](../compatibility/codex.md).

The Claude Code adapter uses the same contract and observation vocabulary
without a Core change; [ADR 0005](0005-claude-code-adapter-boundary.md)
records why its tool-specific rules stay outside broad canonical decisions.
