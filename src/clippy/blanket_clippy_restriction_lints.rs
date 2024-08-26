use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The blanket clippy restriction lints id"]
pub const BLANKET_CLIPPY_RESTRICTION_LINTS_ID: &str = "blanket_clippy_restriction_lints";
#[doc = "The blanket clippy restriction lints description"]
pub const BLANKET_CLIPPY_RESTRICTION_LINTS_DESCRIPTION: &str =
    "Checks for warn/deny/forbid attributes targeting the whole clippy::restriction category.";
#[doc = "The blanket clippy restriction lints know problem"]
pub const BLANKET_CLIPPY_RESTRICTION_LINTS_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The blanket clippy restriction lints what it's bad"]
pub const BLANKET_CLIPPY_RESTRICTION_LINTS_WHATS_BAD: &str = "Restriction lints sometimes are in contrast with other lints or even go against idiomatic rust. These lints should only be enabled on a lint-by-lint basis and with careful consideration.";
#[doc = "The blanket clippy restriction lints uri issue"]
pub const BLANKET_CLIPPY_RESTRICTION_LINTS_ISSUE: Option<&'static str> = Some(
    "https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+blanket_clippy_restriction_lints",
);

#[doc = "The blanket clippy restriction lints for novice"]
pub const NOVICE_BLANKET_CLIPPY_RESTRICTION_LINTS: ClippyLint = ClippyLint {
    id: BLANKET_CLIPPY_RESTRICTION_LINTS_ID,
    description: BLANKET_CLIPPY_RESTRICTION_LINTS_DESCRIPTION,
    whats_bad: BLANKET_CLIPPY_RESTRICTION_LINTS_WHATS_BAD,
    known_problems: BLANKET_CLIPPY_RESTRICTION_LINTS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: BLANKET_CLIPPY_RESTRICTION_LINTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The blanket clippy restriction lints for expert"]
pub const EXPERT_BLANKET_CLIPPY_RESTRICTION_LINTS: ClippyLint = ClippyLint {
    id: BLANKET_CLIPPY_RESTRICTION_LINTS_ID,
    description: BLANKET_CLIPPY_RESTRICTION_LINTS_DESCRIPTION,
    whats_bad: BLANKET_CLIPPY_RESTRICTION_LINTS_WHATS_BAD,
    known_problems: BLANKET_CLIPPY_RESTRICTION_LINTS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: BLANKET_CLIPPY_RESTRICTION_LINTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The blanket clippy restriction lints for master"]
pub const MASTER_BLANKET_CLIPPY_RESTRICTION_LINTS: ClippyLint = ClippyLint {
    id: BLANKET_CLIPPY_RESTRICTION_LINTS_ID,
    description: BLANKET_CLIPPY_RESTRICTION_LINTS_DESCRIPTION,
    whats_bad: BLANKET_CLIPPY_RESTRICTION_LINTS_WHATS_BAD,
    known_problems: BLANKET_CLIPPY_RESTRICTION_LINTS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: BLANKET_CLIPPY_RESTRICTION_LINTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
