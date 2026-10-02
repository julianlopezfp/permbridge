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

> **Current status:** This repository is still a Rust workspace skeleton. It
> does not load YAML policies, import any agent configuration, compare security
> postures, produce diagnostics, intercept actions, mediate approvals, or
> integrate with VS Code. No agent adapter exists. It provides no security
> protection today.

## Product direction

The intended flow is:

```text
Desired policy -> canonical model -> agent adapter -> effective posture
               -> comparison -> diagnostics -> report
```

The canonical model will express desired permissions independently of any
agent. Adapters will interpret supported native configurations and report
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
- A UI-independent `Decision` enum with `Allow`, `Ask`, and `Deny` values,
  defaulting to `Ask` and ordered by restriction.
- Unit tests for that enum, Rust CI, and documentation of the intended model.
- Draft YAML examples and a placeholder for the future TypeScript VS Code
  extension. The examples are not parsed or validated.

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

The planned policy model has a global user policy and an optional project
policy. A project policy may add restrictions but must not weaken a global
restriction. Canonical decisions may be `ALLOW`, `ASK`, and `DENY`, with
`DENY > ASK > ALLOW`; unknown desired actions default to `ASK`. Agent adapters
must not assume that every native permission model has identical states.

The next milestones are a versioned YAML schema and loader, a canonical
capability model, one tested agent adapter, posture comparison, and useful CLI
diagnostics. Additional adapters and a localized VS Code experience are later
work. None of these milestones is implemented in this commit.

## Repository map

| Path | Purpose |
| --- | --- |
| `crates/permbridge-core/` | Canonical Rust types; future comparison logic |
| `crates/permbridge-cli/` | Rust CLI smoke program; future user CLI |
| `adapters/` | Planned agent-specific adapters; no integrations yet |
| `extensions/vscode/` | Future TypeScript extension placeholder |
| `policies/examples/` | Draft, nonfunctional YAML policy examples |
| `docs/` | Canonical English technical documentation |
| `tests/` | Reserved for future cross-crate integration tests |
| `.github/workflows/` | Rust CI |

Read the [architecture](docs/architecture.md) and
[policy concepts](docs/policy-model.md) for the design and its limits. See
[contributing](CONTRIBUTING.md) and [security reporting](SECURITY.md) before
participating. This project is licensed under the [MIT License](LICENSE).
