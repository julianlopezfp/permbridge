# Agent adapters (planned)

This directory reserves implementations of the `permbridge-core::AgentAdapter`
contract. The interface exists, but no concrete agent integration is
implemented. An eventual adapter should describe
the native configuration it can read, the capabilities it can normalize, its
evidence limits, and any unsupported or ambiguous mappings. Agent support
must be claimed only after implementation and behavior tests.
