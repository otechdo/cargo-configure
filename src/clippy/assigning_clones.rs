use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The lints for all"]
pub const ASSIGNING_CLONES_LINTS: (Lint, Lint, Lint) = Lint::new(
    "assigning_clones",
    "Checks for code like foo = bar.clone();",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Pedantic,
    Applicability::Unspecified,
);
