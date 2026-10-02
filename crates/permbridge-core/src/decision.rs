/// A desired action decision. This models permission and approval, not the
/// strength of the mechanism that enforces it.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Decision {
    /// Permit the action without an additional approval step.
    Allow,
    /// Require an approval step before the action proceeds.
    #[default]
    Ask,
    /// Prohibit the action.
    Deny,
}

impl Decision {
    /// Returns the more restrictive decision. This is the combining rule for
    /// comparable global and project decisions; policy merging is not yet
    /// implemented.
    #[must_use]
    pub fn most_restrictive(self, other: Self) -> Self {
        self.max(other)
    }
}

#[cfg(test)]
mod tests {
    use super::Decision;

    #[test]
    fn an_unspecified_decision_requires_approval() {
        assert_eq!(Decision::default(), Decision::Ask);
    }

    #[test]
    fn a_project_decision_cannot_weaken_a_global_decision() {
        use Decision::{Allow, Ask, Deny};

        for (global, project, expected) in [
            (Deny, Allow, Deny),
            (Deny, Ask, Deny),
            (Ask, Allow, Ask),
            (Allow, Ask, Ask),
            (Allow, Deny, Deny),
        ] {
            assert_eq!(global.most_restrictive(project), expected);
            assert_eq!(project.most_restrictive(global), expected);
        }
    }
}
