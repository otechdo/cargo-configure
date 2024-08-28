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
    Applicability::MachineApplicable,
);

#[doc = "The allow attribute for all"]
pub const ALLOW_ATTRIBUTES: (Lint, Lint, Lint) = Lint::new(
    "allow_attributes",
    "Checks for usage of the #[allow] attribute and suggests replacing it with the #[expect]",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::MachineApplicable,
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

#[doc = "assertions_on_constants lint for all"]
pub const ASSERTIONS_ON_CONSTANTS_LINTS: (Lint, Lint, Lint) = Lint::new(
    "assertions_on_constants",
    "Checks for assert!(true) and assert!(false) calls.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Style,
    Applicability::Unspecified,
);

#[doc = "await_holding_lock lint for all"]
pub const AWAIT_HOLDING_LOCK_LINTS: (Lint, Lint, Lint) = Lint::new(
    "await_holding_lock",
    "Checks for calls to await while holding a non-async-aware MutexGuard.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Suspicious,
    Applicability::Unspecified,
);

#[doc = "await_holding_lock lint for all"]
pub const AWAIT_HOLDING_INVALID_TYPE_LINTS: (Lint, Lint, Lint) = Lint::new(
    "await_holding_invalid_type",
    "Allows users to configure types which should not be held across await suspension points.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Suspicious,
    Applicability::Unspecified,
);

#[doc = "assigning_clones lint for all"]
pub const ASSIGN_OP_PATTERN_LINTS: (Lint, Lint, Lint) = Lint::new(
    "assign_op_pattern",
    "Checks for a = a op b or a = b commutative_op a patterns.",
    &Severity::Warn,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Style,
    Applicability::MachineApplicable,
);

#[doc = "assigning_clones lint for all"]
pub const ASYNC_YIELDS_ASYNC_LINTS: (Lint, Lint, Lint) = Lint::new(
    "async_yields_async",
    "Checks for async blocks that yield values of types that can themselves be awaited.",
    &Severity::Deny,
    &Severity::Deny,
    &Severity::Deny,
    ClippyGroup::Correctness,
    Applicability::MaybeIncorrect,
);

#[doc = "let_and_return lint for all"]
pub const LET_AND_RETURN_LINTS: (Lint, Lint, Lint) = Lint::new(
    "let_and_return",
    "Checks for let-bindings, which are subsequently returned.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);

#[doc = "if_let_mutex lint for all"]
pub const IF_LET_MUTEX_LINTS: (Lint, Lint, Lint) = Lint::new(
    "if_let_mutex",
    "Checks for Mutex::lock calls in if let expression with lock calls in any of the else blocks.",
    &Severity::Deny,
    &Severity::Deny,
    &Severity::Deny,
    ClippyGroup::Correctness,
    Applicability::Unspecified,
);
