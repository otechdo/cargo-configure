use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The absolute path lint id"]
pub const ABSOLUTE_PATH_ID: &str = "absolute_paths";

#[doc = "The absolute path lint description"]
pub const ABSOLUTE_PATH_DESCRIPTION: &str =
    "Checks for usage of items through absolute paths, like std::env::current_dir";

#[doc = "The absolute path know problem"]
pub const ABSOLUTE_PATH_KNOW_PROBLEM: Option<&'static str> = Some("There are currently a few cases which are not caught by this lint:\nMacro calls. e.g. path::to::macro!()\nDerive macros. e.g. #[derive(path::to::macro)]\nAttribute macros. e.g. #[path::to::macro]");
#[doc = "The absolute path what it's bad"]
pub const ABSOLUTE_PATH_WHATS_BAD: &str = "Many codebases have their own style when it comes to importing, but one that is seldom used is using absolute paths everywhere.\n\nThis is generally considered unidiomatic, and you should add a use statement.\n\nThe default maximum segments (2) is pretty strict, you may want to increase this in clippy.toml.\nNote: One exception to this is code from macro expansion - this does not lint such cases, as using absolute paths is the proper way of referencing items in one.";
#[doc = "The absolute path uri to issue"]
pub const ABSOLUTE_PATH_ISSUE: Option<&'static str> =
    Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+absolute_paths");

#[doc = "Absolute path lint level for novice"]
pub const NOVICE_ABSOLUTE_PATH: ClippyLint = ClippyLint {
    id: ABSOLUTE_PATH_ID,
    description: ABSOLUTE_PATH_DESCRIPTION,
    whats_bad: ABSOLUTE_PATH_WHATS_BAD,
    known_problems: ABSOLUTE_PATH_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Allow,
    group: LintGroup::Restriction,
    issue: ABSOLUTE_PATH_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
#[doc = "Absolute path lint level for expert"]
pub const EXPERT_ABSOLUTE_PATH: ClippyLint = ClippyLint {
    id: ABSOLUTE_PATH_ID,
    description: ABSOLUTE_PATH_DESCRIPTION,
    whats_bad: ABSOLUTE_PATH_WHATS_BAD,
    known_problems: ABSOLUTE_PATH_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ABSOLUTE_PATH_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "Absolute path lint level for master"]
pub const MASTER_ABSOLUTE_PATH: ClippyLint = ClippyLint {
    id: ABSOLUTE_PATH_ID,
    description: ABSOLUTE_PATH_DESCRIPTION,
    whats_bad: ABSOLUTE_PATH_WHATS_BAD,
    known_problems: ABSOLUTE_PATH_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ABSOLUTE_PATH_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
