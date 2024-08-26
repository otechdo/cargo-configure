use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The as_conversions lint id"]
pub const AS_CONVERSIONS_ID: &str = "as_conversions";
#[doc = "The as_conversions description"]
pub const AS_CONVERSIONS_DESCRIPTION: &str = "Checks for usage of as conversions.\n#\n# Note that this lint is specialized in linting every single use of as regardless of whether good alternatives exist or not.\n#\n# If you want more precise lints for as, please consider using these separate lints: unnecessary_cast, cast_lossless/cast_possible_truncation/cast_possible_wrap/cast_precision_loss/cast_sign_loss, fn_to_numeric_cast(_with_truncation), char_lit_as_u8, ref_to_mut and ptr_as_ptr.\n#\n# There is a good explanation the reason why this lint should work in this way and how it is useful in this issue.\n#";
#[doc = "The as_conversions know problem"]
pub const AS_CONVERSIONS_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The as_conversions what it's bad"]
pub const AS_CONVERSIONS_WHATS_BAD: &str = "The as conversions will perform many kinds of conversions, including silently lossy conversions and dangerous coercions.\n#\n# There are cases when it makes sense to use as, so the lint is Allow by default.";
#[doc = "The as_conversions uri issue"]
pub const AS_CONVERSIONS_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+as_conversions");

#[doc = "The as_conversions for novice"]
pub const NOVICE_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The as_conversions for expert"]
pub const EXPERT_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The as_conversions for master"]
pub const MASTER_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

