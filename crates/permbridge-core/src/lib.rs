//! Shared canonical policy types for PermBridge.
//!
//! Policy loading, agent adapters, and posture comparison are not implemented yet.

/// A desired canonical policy decision. Agent-specific permission states may
/// not map to these variants exactly. The variant order reflects the planned
/// restriction precedence: `Deny` is more restrictive than `Ask`, which is
/// more restrictive than `Allow`.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Decision {
    Allow,
    #[default]
    Ask,
    Deny,
}

#[cfg(test)]
mod tests {
    use super::Decision;

    #[test]
    fn default_decision_is_ask() {
        assert_eq!(Decision::default(), Decision::Ask);
    }

    #[test]
    fn decisions_follow_restriction_order() {
        assert!(Decision::Deny > Decision::Ask);
        assert!(Decision::Ask > Decision::Allow);
    }
}
