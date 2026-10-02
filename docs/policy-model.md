# Policy concepts (design draft)

## Decisions

The intended result for a requested action is `ALLOW`, `ASK`, or `DENY`.
Their restriction order is `DENY > ASK > ALLOW`. The Core currently contains
only a Rust enum with this order and a default value of `Ask`. It does not
evaluate requested actions.

## Sources and combination

The future evaluator will read a global user policy and may read a project
policy. It should calculate each source's decision and choose the more
restrictive one. Thus, a project policy cannot downgrade a global `DENY` to
`ASK` or `ALLOW`, or a global `ASK` to `ALLOW`. A rule that matches no action
should result in `ASK`; no policy file or invalid policy must not silently
produce `ALLOW`. Error handling and policy file locations still need design.

Within a policy, the intended precedence among matching rules is also
`DENY > ASK > ALLOW`. Exact action categories, match fields, validation rules,
and YAML schema are not finalized. The examples in `policies/examples/` are
illustrative drafts and are not parsed or enforced by v0.1.

## Future work needed for enforcement

An enforceable release needs a documented action model, a versioned YAML
schema, strict parsing and validation, reliable interception at integration
boundaries, a safe response to errors, and an explicit approval flow for ASK.
It also needs tests showing that the project policy cannot weaken the global
policy. None of those mechanisms exists in this skeleton.
