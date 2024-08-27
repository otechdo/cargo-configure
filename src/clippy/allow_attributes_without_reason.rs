use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The lints for all"]
pub const ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS: (Lint, Lint, Lint) = Lint::new(
    "allow_attributes_without_reason",
    "Checks for attributes that allow lints without a reason.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);
