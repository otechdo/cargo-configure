use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The almost swapped id"]
pub const ALMOST_SWAPPED_ID: &str = "almost_swapped";
#[doc = "The almost complete range description"]
pub const ALMOST_SWAPPED_DESCRIPTION: &str = "Checks for foo = bar; bar = foo sequences.";
#[doc = "The almost swapped know problem"]
pub const ALMOST_SWAPPED_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The almost swapped what it's bad"]
pub const ALMOST_SWAPPED_WHATS_BAD: &str = "This looks like a failed attempt to swap.";
#[doc = "The almost swapped issue uri"]
pub const ALMOST_SWAPPED_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+almost_swapped");

#[doc = "The almost complete range for novice"]
pub const NOVICE_ALMOST_SWAPPED: ClippyLint = ClippyLint {
    id: ALMOST_SWAPPED_ID,
    description: ALMOST_SWAPPED_DESCRIPTION,
    whats_bad: ALMOST_SWAPPED_WHATS_BAD,
    known_problems: ALMOST_SWAPPED_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ALMOST_SWAPPED_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for expert"]
pub const EXPERT_ALMOST_SWAPPED: ClippyLint = ClippyLint {
    id: ALMOST_SWAPPED_ID,
    description: ALMOST_SWAPPED_DESCRIPTION,
    whats_bad: ALMOST_SWAPPED_WHATS_BAD,
    known_problems: ALMOST_SWAPPED_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ALMOST_SWAPPED_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for master"]
pub const MASTER_ALMOST_SWAPPED: ClippyLint = ClippyLint {
    id: ALMOST_SWAPPED_ID,
    description: ALMOST_SWAPPED_DESCRIPTION,
    whats_bad: ALMOST_SWAPPED_WHATS_BAD,
    known_problems: ALMOST_SWAPPED_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ALMOST_SWAPPED_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};
