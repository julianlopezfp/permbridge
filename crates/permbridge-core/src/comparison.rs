/// Conceptual relationship between desired policy and an agent's observed
/// effective posture for one capability. Core does not calculate it yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonOutcome {
    /// Observed behavior matches the desired rule within stated evidence limits.
    Equivalent,
    /// Observed behavior imposes a stronger restriction.
    MoreRestrictive,
    /// Observed behavior permits more than the desired rule.
    LessRestrictive,
    /// The agent cannot express or expose the rule through its adapter.
    Unsupported,
    /// Available evidence cannot establish a reliable comparison.
    Ambiguous,
}

impl ComparisonOutcome {
    /// Whether the observation is known to meet or exceed the desired
    /// restriction. `None` means the mapping is unsupported or ambiguous;
    /// it must not be treated as a successful comparison.
    #[must_use]
    pub fn meets_or_exceeds_policy(self) -> Option<bool> {
        match self {
            Self::Equivalent | Self::MoreRestrictive => Some(true),
            Self::LessRestrictive => Some(false),
            Self::Unsupported | Self::Ambiguous => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComparisonOutcome;

    #[test]
    fn unsupported_and_ambiguous_mappings_are_not_treated_as_compliant() {
        use ComparisonOutcome::{
            Ambiguous, Equivalent, LessRestrictive, MoreRestrictive, Unsupported,
        };

        assert_eq!(Equivalent.meets_or_exceeds_policy(), Some(true));
        assert_eq!(MoreRestrictive.meets_or_exceeds_policy(), Some(true));
        assert_eq!(LessRestrictive.meets_or_exceeds_policy(), Some(false));
        assert_eq!(Unsupported.meets_or_exceeds_policy(), None);
        assert_eq!(Ambiguous.meets_or_exceeds_policy(), None);
    }
}
