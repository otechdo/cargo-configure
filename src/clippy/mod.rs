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

#[doc = "The approx_constant lint for all"]
pub const APPROX_CONSTANT_LINTS: (Lint, Lint, Lint) = Lint::new(
    "approx_constant",
    "
Checks for floating point literals that approximate constants which are defined in std::f32::consts or std::f64::consts, respectively, suggesting to use the predefined constant.",
    &Severity::Deny,
    &Severity::Deny,
    &Severity::Deny,
    ClippyGroup::Correctness,
    Applicability::Unspecified,
);

#[doc = "arc_with_non_send_sync lint for all"]
pub const ARC_WITH_NO_SEND_LINTS: (Lint, Lint, Lint) = Lint::new(
    "arc_with_non_send_sync",
    "This lint warns when you use Arc with a type that does not implement Send or Sync.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Suspicious,
    Applicability::Unspecified,
);

#[doc = "arithmetic_side_effects lint for all"]
pub const ARITMETIC_SIDE_EFFECTS_LINTS: (Lint, Lint, Lint) = Lint::new(
    "arithmetic_side_effects",
    "Checks any kind of arithmetic operation of any type.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "as_conversions lint for all"]
pub const AS_CONVERSIONS_LINTS: (Lint, Lint, Lint) = Lint::new(
    "as_conversions",
    "Checks for usage of as conversions.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "arithmetic_side_effects lint for all"]
pub const AS_UNDERSCORE_LINTS: (Lint, Lint, Lint) = Lint::new(
    "as_underscore",
    "Checks for usage of as underscore.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::MachineApplicable,
);

#[doc = "as_ptr_cast_mut lint for all"]
pub const AS_PTR_CAST_MUT_LINTS: (Lint, Lint, Lint) = Lint::new(
    "as_ptr_cast_mut",
    "Checks for the result of a &self-taking as_ptr being cast to a mutable pointer.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Nursery,
    Applicability::MaybeIncorrect,
);
