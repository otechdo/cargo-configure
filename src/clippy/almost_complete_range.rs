use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The almost complete range id"]
pub const ALMOST_COMPLETE_RANGE_ID: &str = "almost_complete_range";
#[doc = "The almost complete range description"]
pub const ALMOST_COMPLETE_RANGE_DESCRIPTION: &str =
    "Checks for ranges which almost include the entire range of letters or digits.";
#[doc = "The almost complete range know problem"]
pub const ALMOST_COMPLETE_RANGE_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The almost complete range what it's bad"]
pub const ALMOST_COMPLETE_RANGE_WHATS_BAD: &str =
    "This is almost certainly a typo meant to include all letters.";
#[doc = "The almost complete range issue uri"]
pub const ALMOST_COMPLETE_RANGE_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+almost_complete_range");

#[doc = "The almost complete range for novice"]
pub const NOVICE_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: ALMOST_COMPLETE_RANGE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The almost complete range for expert"]
pub const EXPERT_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The almost complete range for master"]
pub const MASTER_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
