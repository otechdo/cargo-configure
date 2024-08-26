use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The await holding refcell ref id"]
pub const AWAIT_HOLDING_REFCELL_REF_ID: &str = "await_holding_refcell_ref";
#[doc = "The await holding refcell ref description"]
pub const AWAIT_HOLDING_REFCELL_REF_DESCRIPTION: &str = "Checks for calls to await while holding a RefCell, Ref, or RefMut.";
#[doc = "The await holding refcell ref know problem"]
pub const AWAIT_HOLDING_REFCELL_REF_KNOW_PROBLEM: Option<&'static str> = Some("Will report false positive for explicitly dropped guards (#6446). A workaround for this is to wrap the .lock() call in a block instead of explicitly dropping the guard.");
#[doc = "The await holding refcell ref what it's bad"]
pub const AWAIT_HOLDING_REFCELL_REF_WHATS_BAD: &str = "RefCell refs only check for exclusive mutable access at runtime. Holding a RefCell ref across an await suspension point risks panics from a mutable ref shared while other refs are outstanding.";
#[doc = "The await holding refcell ref uri issue"]
pub const AWAIT_HOLDING_REFCELL_REF_LOCK_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+await_holding_refcell_ref");

#[doc = "The await holding refcell ref for novice"]
pub const NOVICE_AWAIT_HOLDING_REFCELL_REF: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_REFCELL_REF_ID,
    description: AWAIT_HOLDING_REFCELL_REF_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_REFCELL_REF_WHATS_BAD,
    known_problems: AWAIT_HOLDING_REFCELL_REF_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_REFCELL_REF_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding refcell ref for expert"]
pub const EXPERT_AWAIT_HOLDING_REFCELL_REF: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_REFCELL_REF_ID,
    description: AWAIT_HOLDING_REFCELL_REF_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_REFCELL_REF_WHATS_BAD,
    known_problems: AWAIT_HOLDING_REFCELL_REF_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_REFCELL_REF_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding refcell ref for master"]
pub const MASTER_AWAIT_HOLDING_REFCELL_REF: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_REFCELL_REF_ID,
    description: AWAIT_HOLDING_REFCELL_REF_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_REFCELL_REF_WHATS_BAD,
    known_problems: AWAIT_HOLDING_REFCELL_REF_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_REFCELL_REF_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};