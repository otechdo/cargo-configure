use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The absurd extreme comparisons lint id"]
pub const ABSURD_EXTREME_COMPARISON_ID: &str = "absurd_extreme_comparisons";

#[doc = "The absurd extreme comparisons description"]
pub const ABSURD_EXTREME_COMPARISON_DESCRIPTION: &str =
    "Checks for usage of items through absolute paths, like std::env::current_dir.";

#[doc = "The absurd extreme comparisons know problem"]
pub const ABSURD_EXTREME_COMPARISON_KNOW_PROBLEM: Option<&'static str> = Some("For usize the size of the current compile target will be assumed (e.g., 64 bits on 64 bit systems). This means code that uses such a comparison to detect target pointer width will trigger this lint. One can use mem::sizeof and compare its value or conditional compilation attributes like #[cfg(target_pointer_width = \"64\")] .. instead.");

#[doc = "The absurd extreme comparisons what it's bad"]
pub const ABSURD_EXTREME_COMPARISON_WHATS_BAD: &str = "An expression like min <= x may misleadingly imply that it is possible for x to be less than the minimum. Expressions like max < x are probably mistakes.";

#[doc = "The absurd extreme comparisons uri issue"]
pub const ABSURD_EXTREME_COMPARISON_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+absurd_extreme_comparisons");

#[doc = "The absurd extreme comparisons for novice"]
pub const NOVICE_ABSURD_EXTREME_COMPARISON: ClippyLint = ClippyLint {
    id: ABSURD_EXTREME_COMPARISON_ID,
    description: ABSURD_EXTREME_COMPARISON_DESCRIPTION,
    whats_bad: ABSURD_EXTREME_COMPARISON_WHATS_BAD,
    known_problems: ABSURD_EXTREME_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ABSURD_EXTREME_COMPARISON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The absurd extreme comparisons for expert"]
pub const EXPERT_ABSURD_EXTREME_COMPARISON: ClippyLint = ClippyLint {
    id: ABSURD_EXTREME_COMPARISON_ID,
    description: ABSURD_EXTREME_COMPARISON_DESCRIPTION,
    whats_bad: ABSURD_EXTREME_COMPARISON_WHATS_BAD,
    known_problems: ABSURD_EXTREME_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ABSURD_EXTREME_COMPARISON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The absurd extreme comparisons for master"]
pub const MASTER_ABSURD_EXTREME_COMPARISON: ClippyLint = ClippyLint {
    id: ABSURD_EXTREME_COMPARISON_ID,
    description: ABSURD_EXTREME_COMPARISON_DESCRIPTION,
    whats_bad: ABSURD_EXTREME_COMPARISON_WHATS_BAD,
    known_problems: ABSURD_EXTREME_COMPARISON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ABSURD_EXTREME_COMPARISON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};
