use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The bool_comparison id"]
pub const BOOL_COMPARISON_ID: &str = "bool_comparison";
#[doc = "The bool_comparison description"]
pub const BOOL_COMPARISON_DESCRIPTION: &str = "Checks for expressions of the form x == true, x != true and order comparisons such as x < true (or vice versa) and suggest using the variable directly.";
#[doc = "The bool_comparison know problem"]
pub const BOOL_COMPARISON_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The bool_comparison what it's bad"]
pub const BOOL_COMPARISON_WHATS_BAD: &str = "Unnecessary code.";
#[doc = "The bool_comparison uri issue"]
pub const BOOL_COMPARISON_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+bool_comparison");

#[doc = "The bool_comparison for novice"]
pub const NOVICE_BOOL_COMPARISON: ClippyLint = ClippyLint {
    id: BOOL_COMPARISON_ID,
    description: BOOL_COMPARISON_DESCRIPTION,
    whats_bad: BOOL_COMPARISON_WHATS_BAD,
    known_problems: BOOL_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Complexity,
    issue: BOOL_COMPARISON_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The bool_comparison for expert"]
pub const EXPERT_BOOL_COMPARISON: ClippyLint = ClippyLint {
    id: BOOL_COMPARISON_ID,
    description: BOOL_COMPARISON_DESCRIPTION,
    whats_bad: BOOL_COMPARISON_WHATS_BAD,
    known_problems: BOOL_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Complexity,
    issue: BOOL_COMPARISON_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The bool_comparison for master"]
pub const MASTER_BOOL_COMPARISON: ClippyLint = ClippyLint {
    id: BOOL_COMPARISON_ID,
    description: BOOL_COMPARISON_DESCRIPTION,
    whats_bad: BOOL_COMPARISON_WHATS_BAD,
    known_problems: BOOL_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Complexity,
    issue: BOOL_COMPARISON_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
