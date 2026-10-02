# Posture comparison

**Status: implemented experimental in-memory API.** `permbridge_core::compare`
takes one `CanonicalPolicy` and one `EffectivePosture`. It returns a
`ComparisonReport` keyed by canonical `Capability`, or an explicit error for
conflicting desired selector rules. It does not load YAML, inspect an agent,
match action requests, merge scopes, or enforce permissions.

The five baseline capabilities are always compared: workspace read, workspace
write, outside-workspace access, default network access, and default command
execution. Domain and command selectors already in the in-memory desired
policy are compared as exact, opaque keys. Equal duplicate desired selectors
coalesce; conflicting duplicates fail. The policy's top-level fallback
`default_decision` does not create additional capabilities in this phase.

## Per-capability result

Each result retains the desired decision, the complete optional adapter
observation, the relationship of mapped decisions when known, the final
outcome, and a stable reason code. Results are stored in a `BTreeMap` for
deterministic ordering. An adapter observation without a corresponding desired
rule is preserved in `unrequested` and receives no inferred comparison. The
posture itself uses a `BTreeMap`, so two simultaneous observations for one
capability are impossible; inserting a second value replaces the first.

Known decisions use the restriction order `Allow < Ask < Deny`. For example,
observed `Deny` against desired `Ask` is more restrictive, while observed
`Allow` is less restrictive. This decision relationship is recorded separately
from the final outcome. It is not computed for missing, unsupported,
not-configured, or ambiguous observations.

An adapter must turn an unknown native value into an ambiguous observation;
the comparator cannot infer the raw native value from `EffectivePosture`.

| Observation | Decision relationship | Final outcome |
| --- | --- | --- |
| `Known` with `ToolMediated` or `OsSandbox` | Equal, more, or less restrictive | `Equivalent`, `MoreRestrictive`, or `LessRestrictive` |
| `Known` with `Declared` | Equal, more, or less restrictive | `Ambiguous` with `DeclaredOnly` reason |
| `Unsupported` | None | `Unsupported` |
| `NotConfigured` | None | `NotConfigured` |
| `Ambiguous` | None | `Ambiguous` |
| No observation | None | `Ambiguous` with `MissingObservation` reason |

`Declared` records file intent only. Even an equal or apparently more
restrictive declared decision cannot pass. `ToolMediated` and `OsSandbox` are
accepted as adapter-reported mechanisms for the specific capability, but the
comparator does not rank these mechanisms against each other. The desired
policy has no enforcement-strength requirement, so no strength equivalence is
claimed. `meets_or_exceeds_policy()` returns `Some(true)` only for adequately
observed equal or more restrictive decisions, `Some(false)` for adequately
observed less restrictive decisions, and `None` for every uncertain outcome.

There is no aggregate pass flag: one passing capability cannot conceal an
unobserved or unsupported one. `Equivalent` describes the relationship under
the supplied adapter evidence, not verified behavior of a running agent.
Both current adapters inspect only selected files. Codex's declared
filesystem allowances therefore retain their decision relationship but yield
an ambiguous final outcome; Claude Code's broad capabilities remain ambiguous
without a decision relationship. Compare each agent independently against the
same desired policy. The report does not rank agents against each other.

See [canonical policy model](canonical-policy-model.md),
[adapter contract](adapter-contract.md), and [security model](security-model.md)
for the limits of these inputs.
