# Architecture and v0.1 scope

## Intended architecture

AgentGuard is a local policy layer between an AI coding agent's requested
action and an eventual decision consumer. The Core is intended to accept a
language-neutral action description and policy data, then return a decision
that a separate integration can present or enforce. The Core must not depend
on VS Code, terminal UI, a particular agent, or localized message strings.

The planned monorepo parts are:

1. `agentguard-core` (Rust): policy parsing, matching, and decision logic.
2. `agentguard-cli` (Rust): a local test and development entry point.
3. `extensions/vscode` (TypeScript): a future editor integration that consumes
   Core decisions and localizes user-facing text.

The first commit only provides the workspace, a `Decision` value type, a CLI
startup smoke test, and tests for that type. No API for action evaluation or
approval exists yet. The extension directory has no package or executable
implementation.

## Security model

The planned trust boundary is local. A future integration will need to ensure
that every relevant agent action is actually presented to the evaluator and
that a DENY result prevents execution. An ASK result will require a separate
approval flow before execution. Neither interception nor approval is present
in v0.1, so this repository currently provides no security protection.

The policy model will combine a global user policy with an optional project
policy. Project rules can increase restrictions only. Unmatched actions will
default to ASK. See [policy model](policy-model.md) for the intended combining
rules and current gaps.

## Internationalization boundary

Decision variants (`Allow`, `Ask`, `Deny`) are stable internal concepts, not
translated UI messages. Future CLI and editor interfaces should provide
English and Spanish messages without embedding localized strings in Core.
The scaffold CLI currently emits only an English development notice and is not
an operational user interface.

## Scope excluded from this commit

- Policy file loading, validation, matching, and precedence evaluation.
- Agent action interception or enforcement.
- ASK approval workflow or persistence.
- VS Code extension implementation or TypeScript build.
- Security guarantees for agent activity.
