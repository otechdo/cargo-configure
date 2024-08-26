use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The bad bit mask id"]
pub const BAD_BIT_MASK_ID: &str = "bad_bit_mask";
#[doc = "The bad bit mask description"]
pub const BAD_BIT_MASK_ID_DESCRIPTION: &str = "Checks for incompatible bit masks in comparisons.";
#[doc = "The bad bit mask know problem"]
pub const BAD_BIT_MASK_ID_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The bad bit mask what it's bad"]
pub const BAD_BIT_MASK_WHATS_BAD: &str = "If the bits that the comparison cares about are always set to zero or one by the bit mask, the comparison is constant true or false (depending on mask, compared value, and operators).\n#\n# So the code is actively misleading, and the only reason someone would write this intentionally is to win an underhanded Rust contest or create a test-case for this lint.";
#[doc = "The bad bit mask uri issue"]
pub const BAD_BIT_MASK_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+bad_bit_mask");

#[doc = "The bad bit mask  for novice"]
pub const NOVICE_BAD_BIT_MASK: ClippyLint = ClippyLint {
    id: BAD_BIT_MASK_ID,
    description: BAD_BIT_MASK_ID_DESCRIPTION,
    whats_bad: BAD_BIT_MASK_WHATS_BAD,
    known_problems: BAD_BIT_MASK_ID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: BAD_BIT_MASK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The bad bit mask  for expert"]
pub const EXPERT_BAD_BIT_MASK: ClippyLint = ClippyLint {
    id: BAD_BIT_MASK_ID,
    description: BAD_BIT_MASK_ID_DESCRIPTION,
    whats_bad: BAD_BIT_MASK_WHATS_BAD,
    known_problems: BAD_BIT_MASK_ID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: BAD_BIT_MASK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The bad bit mask ref for master"]
pub const MASTER_BAD_BIT_MASK: ClippyLint = ClippyLint {
    id: BAD_BIT_MASK_ID,
    description: BAD_BIT_MASK_ID_DESCRIPTION,
    whats_bad: BAD_BIT_MASK_WHATS_BAD,
    known_problems: BAD_BIT_MASK_ID_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: BAD_BIT_MASK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};
