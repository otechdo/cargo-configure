use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The lints for all"]
pub const BLOCKS_IN_CONDITIONS_LINTS: (Lint, Lint, Lint) = Lint::new(
    "blocks_in_conditions",
    "
Checks for if and match conditions that use blocks containing an expression, statements or conditions that use closures with blocks.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Style,
    Applicability::MachineApplicable,
);
