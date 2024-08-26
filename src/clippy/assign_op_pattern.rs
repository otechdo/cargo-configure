use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The assign op pattern id"]
pub const ASSIGN_OF_PATTERN_ID: &str = "assign_op_pattern";
#[doc = "The assign op pattern description"]
pub const ASSIGN_OP_PATTERN_DESCRIPTION: &str =
    "Checks for a = a op b or a = b commutative_op a patterns.";
#[doc = "The assign op pattern know problem"]
pub const ASSIGN_OP_PATTERN_KNOW_PROBLEM: Option<&'static str> = Some("While forbidden by the spec, OpAssign traits may have implementations that differ from the regular Op impl.");
#[doc = "The assign op pattern what it's bad"]
pub const ASSIGN_OP_PATTERN_WHATS_BAD: &str = "These can be written as the shorter a op= b.";
#[doc = "The assign op pattern uri issue"]
pub const ASSIGN_OP_PATTERN_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assign_op_pattern");

#[doc = "The assign op pattern for novice"]
pub const NOVICE_ASSIGN_OP_PATTERN: ClippyLint = ClippyLint {
    id: ASSIGN_OF_PATTERN_ID,
    description: ASSIGN_OP_PATTERN_DESCRIPTION,
    whats_bad: ASSIGN_OP_PATTERN_WHATS_BAD,
    known_problems: ASSIGN_OP_PATTERN_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    issue: ASSIGN_OP_PATTERN_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The assign op pattern for expert"]
pub const EXPERT_ASSIGN_OP_PATTERN: ClippyLint = ClippyLint {
    id: ASSIGN_OF_PATTERN_ID,
    description: ASSIGN_OP_PATTERN_DESCRIPTION,
    whats_bad: ASSIGN_OP_PATTERN_WHATS_BAD,
    known_problems: ASSIGN_OP_PATTERN_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    issue: ASSIGN_OP_PATTERN_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The assign op pattern for master"]
pub const MASTER_ASSIGN_OP_PATTERN: ClippyLint = ClippyLint {
    id: ASSIGN_OF_PATTERN_ID,
    description: ASSIGN_OP_PATTERN_DESCRIPTION,
    whats_bad: ASSIGN_OP_PATTERN_WHATS_BAD,
    known_problems: ASSIGN_OP_PATTERN_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    issue: ASSIGN_OP_PATTERN_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
