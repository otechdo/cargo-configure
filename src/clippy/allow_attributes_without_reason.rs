use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The allow attributes without reason id"]
pub const ALLOW_ATTRIBUTE_WITHOUT_REASON_ID: &str = "allow_attributes_without_reason";
#[doc = "The allow attributes without reason description"]
pub const ALLOW_ATTRIBUTE_WITHOUT_REASON_DESCRIPTION: &str =
    "Checks for attributes that allow lints without a reason.";
#[doc = "The allow attributes without reason know problem"]
pub const ALLOW_ATTRIBUTE_WITHOUT_REASON_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The allow attributes without reason what it's bad"]
pub const ALLOW_ATTRIBUTE_WITHOUT_REASON_WHATS_BAD: &str = "Justifying each allow helps readers understand the reasoning, and may allow removing allow attributes if their purpose is obsolete.";
#[doc = "The allow attributes without reason issue uri"]
pub const ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+allow_attributes_without_reason");

#[doc = "The allow attributes without reason lint for novice"]
pub const NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_WITHOUT_REASON_ID,
    description: ALLOW_ATTRIBUTE_WITHOUT_REASON_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WITHOUT_REASON_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_WITHOUT_REASON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The allow attributes without reason lint for expert"]
pub const EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_WITHOUT_REASON_ID,
    description: ALLOW_ATTRIBUTE_WITHOUT_REASON_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WITHOUT_REASON_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_WITHOUT_REASON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The allow attributes lint for for master"]
pub const MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_WITHOUT_REASON_ID,
    description: ALLOW_ATTRIBUTE_WITHOUT_REASON_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WITHOUT_REASON_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_WITHOUT_REASON_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
