use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The blanket_clippy_restriction_lints id"]
pub const BLOCK_IN_CONDITION_ID: &str = "blanket_clippy_restriction_lints";
#[doc = "The blanket_clippy_restriction_lints description"]
pub const BLOCK_IN_CONDITION_DESCRIPTION: &str = "Checks for if and match conditions that use blocks containing an expression, statements or conditions that use closures with blocks.";
#[doc = "The blanket_clippy_restriction_lints know problem"]
pub const BLOCK_IN_CONDITION_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The blanket_clippy_restriction_lints what it's bad"]
pub const BLOCK_IN_CONDITION_WHATS_BAD: &str =
    "Style, using blocks in the condition makes it hard to read.";
#[doc = "The blanket_clippy_restriction_lints uri issue"]
pub const BLOCK_IN_CONDITION_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+blocks_in_conditions");

#[doc = "The blanket_clippy_restriction_lints for novice"]
pub const NOVICE_BLOCK_IN_CONDITION: ClippyLint = ClippyLint {
    id: BLOCK_IN_CONDITION_ID,
    description: BLOCK_IN_CONDITION_DESCRIPTION,
    whats_bad: BLOCK_IN_CONDITION_WHATS_BAD,
    known_problems: BLOCK_IN_CONDITION_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    issue: BLOCK_IN_CONDITION_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The blanket_clippy_restriction_lints for expert"]
pub const EXPERT_BLOCK_IN_CONDITION: ClippyLint = ClippyLint {
    id: BLOCK_IN_CONDITION_ID,
    description: BLOCK_IN_CONDITION_DESCRIPTION,
    whats_bad: BLOCK_IN_CONDITION_WHATS_BAD,
    known_problems: BLOCK_IN_CONDITION_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    issue: BLOCK_IN_CONDITION_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
#[doc = "The blanket_clippy_restriction_lints for master"]
pub const MASTER_BLOCK_IN_CONDITION: ClippyLint = ClippyLint {
    id: BLOCK_IN_CONDITION_ID,
    description: BLOCK_IN_CONDITION_DESCRIPTION,
    whats_bad: BLOCK_IN_CONDITION_WHATS_BAD,
    known_problems: BLOCK_IN_CONDITION_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    issue: BLOCK_IN_CONDITION_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
