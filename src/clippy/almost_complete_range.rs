use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The lints for all"]
pub const ALMOST_COMPLETE_RANGE_LINTS: (Lint, Lint, Lint) = Lint::new(
    "almost_complete_range",
    "Checks for ranges which almost include the entire range of letters or digits.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);
