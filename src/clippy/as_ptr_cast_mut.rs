use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The as ptr cast mut id"]
pub const AS_PTR_CAST_MUT_ID: &str = "as_ptr_cast_mut";
#[doc = "The as ptr cast mut description"]
pub const AS_PTR_CAST_MUT_DESCRIPTION: &str =
    "Checks for the result of a &self-taking as_ptr being cast to a mutable pointer.";
#[doc = "The as ptr cast mut know problem"]
pub const AS_PTR_CAST_MUT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The as ptr cast mut what it's bad"]
pub const AS_PTR_CAST_MUT_WHATS_BAD: &str = "Since as_ptr takes a &self, the pointer no have write permissions unless interior mutability is used, making it unlikely that having it as a mutable pointer is correct.";
#[doc = "The as ptr cast mut uri issue"]
pub const AS_PTR_CAST_MUT_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+as_ptr_cast_mut");

#[doc = "The as ptr cast mut for novice"]
pub const NOVICE_AS_PTR_CAST_MUT: ClippyLint = ClippyLint {
    id: AS_PTR_CAST_MUT_ID,
    description: AS_PTR_CAST_MUT_DESCRIPTION,
    whats_bad: AS_PTR_CAST_MUT_WHATS_BAD,
    known_problems: AS_PTR_CAST_MUT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Allow,
    group: LintGroup::Nursery,
    issue: AS_PTR_CAST_MUT_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The as ptr cast mut for expert"]
pub const EXPERT_AS_PTR_CAST_MUT: ClippyLint = ClippyLint {
    id: AS_PTR_CAST_MUT_ID,
    description: AS_PTR_CAST_MUT_DESCRIPTION,
    whats_bad: AS_PTR_CAST_MUT_WHATS_BAD,
    known_problems: AS_PTR_CAST_MUT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Nursery,
    issue: AS_PTR_CAST_MUT_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The as ptr cast mut for master"]
pub const MASTER_AS_PTR_CAST_MUT: ClippyLint = ClippyLint {
    id: AS_PTR_CAST_MUT_ID,
    description: AS_PTR_CAST_MUT_DESCRIPTION,
    whats_bad: AS_PTR_CAST_MUT_WHATS_BAD,
    known_problems: AS_PTR_CAST_MUT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Nursery,
    issue: AS_PTR_CAST_MUT_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
