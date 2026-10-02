# Policy and posture comparison concepts

This document describes the planned model. No policy schema, parser, adapter,
or comparator is implemented in the current skeleton.

## Canonical desired policy

A future YAML policy will describe desired capabilities without depending on
an agent's native configuration syntax. A canonical decision may be `ALLOW`,
`ASK`, or `DENY`. The intended restriction order is `DENY > ASK > ALLOW` and
unknown or unmatched desired actions default to `ASK`. The Rust Core currently
provides only a `Decision` enum with this order and default; it does not load
or evaluate policies.

The future evaluator will combine a global user policy with an optional
project policy. For a comparable capability, the more restrictive decision
must win. A project policy must not downgrade a global `DENY` to `ASK` or
`ALLOW`, or a global `ASK` to `ALLOW`. Missing, invalid, and conflicting
policy behavior requires an explicit versioned specification before parsing
is implemented.

## Effective posture and agent adapters

An adapter will import an agent's native configuration, normalize only
supported permissions, and report evidence and capability limits. Native
approval states may not be equivalent to canonical `ASK`; native allow and
deny rules may have different scope or enforcement guarantees. A mapping must
preserve such differences instead of assuming that all agents implement the
three canonical states identically.

No agent adapter exists yet. The `adapters/` directory records the intended
boundary without claiming support for Codex, Claude Code, Cursor, or any
other agent.

## Comparison outcomes

The planned comparison vocabulary includes:

| Outcome | Meaning |
| --- | --- |
| `EQUIVALENT` | Observed effective behavior matches the desired capability within the adapter's stated evidence limits. |
| `MORE_RESTRICTIVE` | The observed behavior imposes a stronger restriction. |
| `LESS_RESTRICTIVE` | The observed behavior permits more than intended. |
| `UNSUPPORTED` | The agent cannot express or verify the desired rule through the adapter. |
| `AMBIGUOUS` | Available evidence cannot establish a reliable ordering or mapping. |

These are conceptual outcomes, not values currently produced by code. A
future diagnostics engine should explain the evidence, gaps, and possible
remediation for each finding. Unsupported and ambiguous results must remain
visible rather than being treated as equivalent.

## Draft YAML examples

The examples in `policies/examples/` illustrate desired permissions and
comparison validation preferences. Their field names and meanings may change
when the versioned schema is defined. They are not parsed, validated, or
enforced by this repository.
