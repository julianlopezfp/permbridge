[English](README.md) | [Español](README.es.md)

# PermBridge

**PermBridge means Permission Bridge.** PermBridge is a developer-first policy
compatibility layer for AI coding agents. Define permissions once, compare how
different agents enforce them, detect security gaps and unsupported rules, and
maintain a consistent security posture across coding tools.

Coding agents can have different permission models, configuration formats,
approval mechanisms, and security guarantees. Switching agents can therefore
change a developer's effective security posture. PermBridge is designed to
make those differences visible and actionable.

> **Current status:** Core now has an experimental in-memory policy model and
> agent adapter contract. It does not load YAML, import any real agent
> configuration, compare postures, produce diagnostics, intercept actions,
> mediate approvals, or integrate with VS Code. No real adapter exists, and
> PermBridge provides no security protection today.

![PermBridge concept artwork](images/permbridge-github-social-preview.jpg)

The image is concept artwork for the planned product, not a claim that its
depicted capabilities are implemented.

## Product direction

The intended flow is:

```text
Desired policy -> canonical model -> agent adapter -> effective posture
               -> comparison -> diagnostics -> report
```

The canonical model expresses desired permissions independently of any
agent. Future adapters will interpret supported native configurations and report
their limitations. A comparator will identify equivalent, more restrictive,
less restrictive, unsupported, and ambiguous mappings. Diagnostics and reports
will explain gaps and possible remediation. The CLI is the first planned user
experience; the VS Code extension comes later.

PermBridge is focused on policy compatibility, posture comparison, and
portability across coding agents. Enforcement or mediation is only a future
possibility where an underlying agent provides a reliable mechanism. This is
not a universal sandbox or a general AI-agent firewall.

## What is implemented

- A Rust workspace containing `permbridge-core` and a `permbridge-cli` smoke
  program.
- An experimental, UI-independent in-memory policy model for filesystem,
  network, and execution decisions, plus policy scope and enforcement strength.
- An `AgentAdapter` trait and observation types that can preserve unsupported
  and ambiguous mappings. No concrete agent adapter implements the trait.
- Comparison outcome vocabulary and focused unit tests for the implemented
  invariants. No comparator calculates outcomes yet.
- Rust CI, illustrative YAML examples, and a placeholder for the future
  TypeScript VS Code extension. The YAML is not parsed or validated.

The CLI only prints a scaffold notice. No comparison result is calculated.

## Development

Install stable Rust with the `rustfmt` and `clippy` components. Node.js and
VS Code are not needed for this skeleton.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --all-targets
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo run -p permbridge-cli
```

The last command is a smoke test, not an operational CLI. There is no
installation or configuration procedure for end users yet. The YAML under
`policies/examples/` illustrates an evolving policy format and has no effect.

## Policy and roadmap

The model represents managed, global, project, and session policy scopes.
Global and optional project policies are intended to combine without allowing
the project policy to weaken a global restriction. Canonical decisions are
`ALLOW`, `ASK`, and `DENY`, with `DENY > ASK > ALLOW`; new policies default to
`ASK`. Merging scopes and evaluating actions are not implemented. Agent
adapters must not assume every native permission model has identical states.

The next milestones are a versioned YAML schema and loader, one tested agent
adapter, posture comparison, and useful CLI diagnostics. Additional adapters
and a localized VS Code experience are later work. See the
[roadmap](docs/roadmap.md) for explicit boundaries.

## Repository map

| Path | Purpose |
| --- | --- |
| `crates/permbridge-core/` | Canonical Rust model and adapter contract; future comparison logic |
| `crates/permbridge-cli/` | Rust CLI smoke program; future user CLI |
| `adapters/` | Home for future agent-specific implementations; none yet |
| `extensions/vscode/` | Future TypeScript extension placeholder |
| `policies/examples/` | Draft, nonfunctional YAML policy examples |
| `docs/` | Canonical English technical documentation |
| `tests/` | Reserved for future cross-crate integration tests |
| `.github/workflows/` | Rust CI |

Read the [architecture](docs/architecture.md),
[canonical policy model](docs/canonical-policy-model.md), and
[adapter contract](docs/adapter-contract.md) for the design and its limits.
The [documentation index](docs/README.md) lists the other engineering guides. See
[contributing](CONTRIBUTING.md) and [security reporting](SECURITY.md) before
participating. This project is licensed under the [MIT License](LICENSE).
