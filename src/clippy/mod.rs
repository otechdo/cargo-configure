use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The absolute path lints for all"]
pub const ABSOLUTE_PATH_LINTS: (Lint, Lint, Lint) = Lint::new(
    "absolute_paths",
    "Checks for usage of items through absolute paths, like std::env::current_dir.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "The allow attribute without reason lints for all"]
pub const ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS: (Lint, Lint, Lint) = Lint::new(
    "allow_attributes_without_reason",
    "Checks for attributes that allow lints without a reason.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "The almost complete range lints for all"]
pub const ALMOST_COMPLETE_RANGE_LINTS: (Lint, Lint, Lint) = Lint::new(
    "almost_complete_range",
    "Checks for ranges which almost include the entire range of letters or digits.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "The also swaped lints for all"]
pub const ALMOST_SWAPED_LINTS: (Lint, Lint, Lint) = Lint::new(
    "almost_swaped",
    "Checks for foo = bar; bar = foo sequences.",
    &Severity::Deny,
    &Severity::Deny,
    &Severity::Deny,
    ClippyGroup::Style,
    Applicability::MachineApplicable,
);

#[doc = "The assigning clone lints for all"]
pub const ASSIGNING_CLONES_LINTS: (Lint, Lint, Lint) = Lint::new(
    "assigning_clones",
    "Checks for code like foo = bar.clone();",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Pedantic,
    Applicability::Unspecified,
);

#[doc = "The block in condition lints for all"]
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
