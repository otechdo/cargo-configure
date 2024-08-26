use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The arithmetic side effects id"]
pub const ARITHMETIC_SIDE_EFFECT_ID: &str = "arithmetic_side_effects";
#[doc = "The arithmetic side effects description"]
pub const ARITHMETIC_SIDE_EFFECT_DESCRIPTION: &str = "Checks any kind of arithmetic operation of any type.";

#[doc = "The arithmetic side effects know problem"]
pub const ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The arithmetic side effects whats bad"]
pub const ARITHMETIC_SIDE_EFFECT_WHATS_BAD: &str = "For integers, overflow will trigger a panic in debug builds or wrap the result in release mode; division by zero will cause a panic in either mode. As a result, it is desirable to explicitly call checked, wrapping or saturating arithmetic methods.";
#[doc = "The arc with non send sync issue uri"]
pub const ARITHMETIC_SIDE_EFFECT_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+arithmetic_side_effects");

#[doc = "The arithmetic side effects for novice"]
pub const NOVICE_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The arithmetic side effects for expert"]
pub const EXPERT_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The arithmetic side effects for master"]
pub const MASTER_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};