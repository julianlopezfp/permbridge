# Agent adapter contract

**Status: experimental contract with limited Codex and Claude Code implementations.**

`AgentAdapter` exposes two operations: `id()` returns a stable integration
identifier, and `inspect(&CanonicalPolicy)` returns an `EffectivePosture` or
an adapter-specific error. Passing the desired policy lets an adapter focus on
relevant capabilities and mark rules it cannot translate. The separate Core
[comparator](comparison.md) decides the final outcome.

An `EffectivePosture` maps canonical `Capability` keys to observations:

| Observation | Meaning |
| --- | --- |
| `Known` | A native decision and its observed enforcement strength can be described. This does not imply equivalence to the desired rule. |
| `NotConfigured` | Inspected sources contain no explicit setting for this capability; no default is inferred. |
| `Unsupported` | The agent cannot express or expose the requested capability through this adapter. |
| `Ambiguous` | Available native settings cannot be mapped with confidence. |
| Missing key | The capability was not inspected; the comparator reports an ambiguous outcome with a missing-observation reason. |

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
limits before PermBridge claims broad support for an agent. The Core test
double remains separate from the experimental, read-only
[Codex adapter](compatibility/codex.md). Both adapters have an `inspect_report`
method that adds source provenance without changing the provider-agnostic
trait. `NotConfigured` distinguishes a checked but absent setting from a
capability that was never inspected. The
[Claude Code adapter](compatibility/claude-code.md) uses the same vocabulary
without a Core change: its tool-specific rules do not establish broad
canonical decisions from static files.

The adapter contract describes agent capabilities, not rights to use
PermBridge. A future third-party integration must respect the project's
[license and contribution policy](licensing.md).
