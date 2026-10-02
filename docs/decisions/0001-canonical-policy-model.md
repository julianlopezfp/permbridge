# ADR 0001: Use a canonical policy model

## Status

Accepted, 2026-10-02. The type surface is experimental.

## Context

Coding agents express permissions with different concepts and levels of
enforcement. Direct pairwise translation would require a new conversion for
each agent pair and make it difficult to state one developer intent across
tools. A translation can also conceal an unsupported or weaker native rule.

## Decision

Represent desired permissions in an agent-agnostic in-memory model. Each
adapter maps relevant native behavior into explicit observations. A separate
comparator assesses those observations against the desired model. An
adapter may report unsupported or ambiguous mappings instead of inventing an
equivalent native setting.

## Consequences

The shared model gives comparisons a stable point of reference and avoids a
matrix of pairwise converters. It also requires careful specification of
capabilities and selector semantics; the current Rust types and YAML examples
are not a complete or stable policy language. Some native capabilities may
remain unrepresentable and should be reported as such.
