# Agent adapter contract

**Status: experimental contract; no real adapter is implemented.**

`AgentAdapter` exposes two operations: `id()` returns a stable integration
identifier, and `inspect(&CanonicalPolicy)` returns an `EffectivePosture` or
an adapter-specific error. Passing the desired policy lets an adapter focus on
relevant capabilities and mark rules it cannot translate. It must not decide
the final comparison outcome.

An `EffectivePosture` maps canonical `Capability` keys to observations:

| Observation | Meaning |
| --- | --- |
| `Known` | A native decision and its observed enforcement strength can be described. This does not imply equivalence to the desired rule. |
| `Unsupported` | The agent cannot express or expose the requested capability through this adapter. |
| `Ambiguous` | Available native settings cannot be mapped with confidence. |
| Missing key | The capability was not inspected; a future comparator must not assume compliance. |

An inspection error means the adapter could not produce a reliable posture at
all. Its associated Rust error type keeps failures explicit without forcing a
shared error hierarchy before real integrations reveal common cases.

`Capability` includes baseline filesystem, network, and execution variants,
plus opaque domain and command selectors. No syntax, normalization, or matching
semantics are promised yet. Adapters should not silently interpret selectors
more broadly or narrowly than the eventual policy specification.

Enforcement strength distinguishes a stored declaration, a tool or approval
gate, and an OS sandbox boundary for a specific capability. It is descriptive
evidence, not a universal ranking or proof that all actions are contained.
Real adapters will need tests against native behavior and documented evidence
limits before PermBridge claims support for an agent. The current test double
exists only in Core unit tests.
