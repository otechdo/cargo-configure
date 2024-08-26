use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The assigning clones id"]
pub const ASSIGNING_CLONE_ID: &str = "assigning_clones";
#[doc = "The assigning clones description"]
pub const ASSIGNING_CLONE_DESCRIPTION: &str = "Checks for code like foo = bar.clone();";
#[doc = "The assigning clones know problem"]
pub const ASSIGNING_CLONE_KNOW_PROBLEM: Option<&'static str> = Some("While forbidden by the spec, OpAssign traits may have implementations that differ from the regular Op impl.");
#[doc = "The assigning clones what it's bad"]
pub const ASSIGNING_CLONE_WHATS_BAD: &str = "Custom Clone::clone_from() or ToOwned::clone_into implementations allow the objects to share resources and therefore avoid allocations.";
#[doc = "The assigning clones uri issue"]
pub const ASSIGNING_CLONE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assigning_clones");

#[doc = "The assigning clones for novice"]
pub const NOVICE_ASSIGNING_CLONE: ClippyLint = ClippyLint {
    id: ASSIGNING_CLONE_ID,
    description: ASSIGNING_CLONE_DESCRIPTION,
    whats_bad: ASSIGNING_CLONE_WHATS_BAD,
    known_problems: ASSIGNING_CLONE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Pedantic,
    issue: ASSIGNING_CLONE_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assigning clones for expert"]
pub const EXPERT_ASSIGNING_CLONE: ClippyLint = ClippyLint {
    id: ASSIGNING_CLONE_ID,
    description: ASSIGNING_CLONE_DESCRIPTION,
    whats_bad: ASSIGNING_CLONE_WHATS_BAD,
    known_problems: ASSIGNING_CLONE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Pedantic,
    issue: ASSIGNING_CLONE_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assigning clones for master"]
pub const MASTER_ASSIGNING_CLONE: ClippyLint = ClippyLint {
    id: ASSIGNING_CLONE_ID,
    description: ASSIGNING_CLONE_DESCRIPTION,
    whats_bad: ASSIGNING_CLONE_WHATS_BAD,
    known_problems: ASSIGNING_CLONE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Pedantic,
    issue: ASSIGNING_CLONE_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
