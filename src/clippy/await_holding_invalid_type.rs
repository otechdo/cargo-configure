use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The await holding invalid type id"]
pub const AWAIT_HOLDING_INVALID_TYPE_ID: &str = "await_holding_invalid_type";
#[doc = "The await holding invalid type description"]
pub const AWAIT_HOLDING_INVALID_DESCRIPTION: &str = "Allows users to configure types which should not be held across await suspension points.";
#[doc = "The await holding invalid type problem"]
pub const AWAIT_HOLDING_INVALID_KNOW_PROBLEM: Option<&'static str> = Some("While forbidden by the spec, OpAssign traits may have implementations that differ from the regular Op impl.");
#[doc = "The await holding invalid type what it's bad"]
pub const AWAIT_HOLDING_INVALID_WHATS_BAD: &str = "There are some types which are perfectly safe to use concurrently from a memory access perspective, but that will cause bugs at runtime if they are held in such a way.";
#[doc = "The await holding invalid type uri issue"]
pub const AWAIT_HOLDING_INVALID_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+await_holding_invalid_type");
#[doc = "The await holding invalid for novice"]
pub const NOVICE_AWAIT_HOLDING_INVALID: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_INVALID_TYPE_ID,
    description: AWAIT_HOLDING_INVALID_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_INVALID_WHATS_BAD,
    known_problems: AWAIT_HOLDING_INVALID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_INVALID_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding invalid for expert"]
pub const EXPERT_AWAIT_HOLDING_INVALID: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_INVALID_TYPE_ID,
    description: AWAIT_HOLDING_INVALID_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_INVALID_WHATS_BAD,
    known_problems: AWAIT_HOLDING_INVALID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_INVALID_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding invalid for master"]
pub const MASTER_AWAIT_HOLDING_INVALID: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_INVALID_TYPE_ID,
    description: AWAIT_HOLDING_INVALID_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_INVALID_WHATS_BAD,
    known_problems: AWAIT_HOLDING_INVALID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_INVALID_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
