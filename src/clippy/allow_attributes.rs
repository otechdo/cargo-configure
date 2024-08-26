use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The allow attributes id"]
pub const ALLOW_ATTRIBUTE_ID: &str = "allow_attributes";
#[doc = "The allow attributes description"]
pub const ALLOW_ATTRIBUTE_DESCRIPTION: &str =
    "Checks for usage of the #[allow] attribute and suggests replacing it with the #[expect]";

#[doc = "The allow attributes know problem"]
pub const ALLOW_ATTRIBUTE_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The allow attributes what it's bad"]
pub const ALLOW_ATTRIBUTE_WHATS_BAD: &str = "#[expect] attributes suppress the lint emission, but emit a warning, if the expectation is unfulfilled. This can be useful to be notified when the lint is no longer triggered.";
#[doc = "The  allow attributes uri issue"]
pub const ALLOW_ATTRIBUTE_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+allow_attributes");

#[doc = "The allow attributes lint for novice"]
pub const NOVICE_ALLOW_ATTRIBUTE: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_ID,
    description: ALLOW_ATTRIBUTE_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The allow attributes lint for expert"]
pub const EXPERT_ALLOW_ATTRIBUTE: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_ID,
    description: ALLOW_ATTRIBUTE_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The allow attributes lint for master"]
pub const MASTER_ALLOW_ATTRIBUTE: ClippyLint = ClippyLint {
    id: ALLOW_ATTRIBUTE_ID,
    description: ALLOW_ATTRIBUTE_DESCRIPTION,
    whats_bad: ALLOW_ATTRIBUTE_WHATS_BAD,
    known_problems: ALLOW_ATTRIBUTE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ALLOW_ATTRIBUTE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
