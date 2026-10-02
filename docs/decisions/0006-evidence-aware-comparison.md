# ADR 0006: Compare decisions only with capability-specific mechanism evidence

## Status

Accepted, 2026-10-02. The comparator API is experimental.

## Context

Core already ordered `Allow < Ask < Deny` and named five comparison outcomes,
but it did not calculate results. Codex can report declared allowances from
selected files; Claude Code reports uncertainty for broad capabilities. A
decision match alone would turn static file intent into a passing result.
`EnforcementStrength` distinguishes a declaration, tool mediation, and an OS
sandbox, but the latter two mechanisms do not form a universal total order.

## Decision

Compare one desired canonical policy with one agent posture, capability by
capability. Preserve the mapped decision relationship independently of the
final outcome. `Known` observations with only `Declared` strength yield an
ambiguous final outcome, even when decisions match. `ToolMediated` and
`OsSandbox` observations may produce ordered outcomes for their specific
capability; the comparator does not rank their strengths or claim runtime
verification. Missing and uncertain observations never pass.

Add `NotConfigured` to the outcome vocabulary and stable reason codes.
Preserve unrequested observations separately, reject conflicting desired
selectors, and omit an aggregate verdict that could hide partial evidence.
No provider-specific type enters Core.

## Consequences

Both current adapters can feed the same API, but their file-only results do
not produce a passing security claim. A future policy model may need an
explicit required mechanism or finer capability selectors before comparing
strength across agents. Native behavior tests remain necessary before an
operational security conclusion. See [comparison](../comparison.md).
