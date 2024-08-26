use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The await holding lock id"]
pub const AWAIT_HOLDING_LOCK_ID: &str = "await_holding_lock";
#[doc = "The await holding lock description"]
pub const AWAIT_HOLDING_LOCK_DESCRIPTION: &str = "Checks for calls to await while holding a non-async-aware MutexGuard.";
#[doc = "The await holding lock know problem"]
pub const AWAIT_HOLDING_LOCK_KNOW_PROBLEM: Option<&'static str> = Some("Will report false positive for explicitly dropped guards (#6446). A workaround for this is to wrap the .lock() call in a block instead of explicitly dropping the guard.");
#[doc = "The await holding lock what it's bad"]
pub const AWAIT_HOLDING_LOCK_WHATS_BAD: &str = "The Mutex types found in [std::sync][https://doc.rust-lang.org/stable/std/sync/] and parking_lot are not designed to operate in an async context across await points.\n#\n# There are two potential solutions. One is to use an async-aware Mutex type. Many asynchronous foundation crates provide such a Mutex type. The other solution is to ensure the mutex is unlocked before calling await, either by introducing a scope or an explicit call to Drop::drop.";
#[doc = "The await holding lock uri issue"]
pub const AWAIT_HOLDING_LOCK_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+await_holding_lock");

#[doc = "The await holding lock for novice"]
pub const NOVICE_AWAIT_HOLDING_LOCK: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_LOCK_ID,
    description: AWAIT_HOLDING_LOCK_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_LOCK_WHATS_BAD,
    known_problems: AWAIT_HOLDING_LOCK_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding lock for expert"]
pub const EXPERT_AWAIT_HOLDING_LOCK: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_LOCK_ID,
    description: AWAIT_HOLDING_LOCK_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_LOCK_WHATS_BAD,
    known_problems: AWAIT_HOLDING_LOCK_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The await holding lock for master"]
pub const MASTER_AWAIT_HOLDING_LOCK: ClippyLint = ClippyLint {
    id: AWAIT_HOLDING_LOCK_ID,
    description: AWAIT_HOLDING_LOCK_DESCRIPTION,
    whats_bad: AWAIT_HOLDING_LOCK_WHATS_BAD,
    known_problems: AWAIT_HOLDING_LOCK_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    issue: AWAIT_HOLDING_LOCK_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};