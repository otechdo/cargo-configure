use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
#[doc = "All developer level"]
pub enum ClippyLevel {
    Novice,
    Expert,
    Master,
}

#[doc = "All clippy supported lints"]
pub enum Lint {
    AbsolutePaths,
    AbsurdExtremeComparison,
    AllocInsteadOfCore,
    AllowAttribute,
    AllowAttributeWithoutReason,
    AlmostCompleteRange,
    AlmostSwapped,
    ApproxConstant,
    ArcWithNoSendSync,
    ArithmeticsSideEffects,
}

impl Display for ClippyLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClippyLevel::Novice => write!(f, "novice"),
            ClippyLevel::Expert => write!(f, "expert"),
            ClippyLevel::Master => write!(f, "master"),
        }
    }
}
#[doc = "All severity level"]
pub enum LintSeverity<'a> {
    Allow,
    Warn,
    Deny,
    Increase(&'a Self),
    Explain(&'a Self, &'a Lint, &'a ClippyLevel),
    Decrease(&'a Self),
}
impl<'a> Display for LintSeverity<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LintSeverity::Allow => write!(f, "allow"),
            LintSeverity::Warn => write!(f, "warn"),
            LintSeverity::Deny => write!(f, "deny"),
            LintSeverity::Increase(inner) => match inner {
                LintSeverity::Allow => write!(f, "values : (warn, deny)"),
                LintSeverity::Warn => write!(f, "value : deny"),
                LintSeverity::Deny => write!(f, "value none"),
                _ => write!(f, "value : deny"),
            },
            LintSeverity::Decrease(inner) => match inner {
                LintSeverity::Allow => write!(f, "value none"),
                LintSeverity::Warn => write!(f, "value : allow"),
                LintSeverity::Deny => write!(f, "values : (allow, warn)"),
                _ => writeln!(f, "value : allow"),
            },
            LintSeverity::Explain(inner, choose, level) => match inner {
                LintSeverity::Allow => match choose {
                    Lint::AbsolutePaths => match level {
                        ClippyLevel::Novice => write!(f, "\n# Why {inner} for {level} ?\n#\n# 1) Learning Curve:\n#\n# For newcomers to Rust and Clippy, understanding the nuances of relative paths and module systems can be challenging.\n# Enforcing strict rules about absolute paths at the Novice level might create unnecessary friction and frustration for beginners.\n# \n# 2) Simplicity:\n#\n# In some simple projects or learning exercises, using absolute paths might be the most straightforward way to reference files or modules.\n# Forcing relative paths in these cases could add complexity without significant benefit.\n#\n# 3) Focus on Fundamentals:\n#\n# At the Novice level, the priority is likely to be on helping users grasp core Rust concepts and avoid common errors.\n#\n# The AbsolutePaths lint, while valuable for code maintainability, might be considered a secondary concern at this stage.\n#\n# 4) Gradual Introduction of Best Practices:\n#\n# Clippy often adopts a progressive approach, introducing more advanced and nuanced lints as the user's proficiency increases.\n#\n# By setting AbsolutePaths to Allow for Novice, Clippy encourages users to focus on the basics first and gradually adopt best practices as they gain experience.\n#"),
                        ClippyLevel::Expert => write!(f, "\n# Why {inner} for {level} ?\n#\n# 1) Contextual Exceptions:\n#\n# There might be specific situations where using absolute paths is justified or even necessary, even for experienced Rust developers. For instance:\n#\n# Test Suites: In some testing frameworks, it might be more convenient or reliable to use absolute paths to reference test data or configuration files.\n# Legacy Code: When working with existing codebases, it might be impractical or time-consuming to refactor all absolute paths to relative ones immediately.\n#\n# Allowing them temporarily at the Expert level can facilitate incremental improvements.\n#\n# Specific Project Requirements: Certain project structures or build systems might benefit from the use of absolute paths in specific cases.\n#\n# 2) Developer Choice:\n#\n# The Expert level might be designed to give experienced developers more control over their code and allow them to make informed decisions about when to deviate from strict linting rules.\n#\n# 3) Lint Maturity:\n#\n# Some lints, including AbsolutePaths, might still be under development or refinement.\n#\n# Setting them to Allow at the Expert level can help gather feedback from experienced users and identify potential false positives or edge cases.\n#\n# 4) Balancing Strictness and Productivity:\n#\n# While encouraging best practices is important, overly strict linting rules can sometimes hinder productivity or lead to unnecessary code churn.\n#\n# Allowing some flexibility at the Expert level can strike a balance between code quality and developer efficiency.\n#"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AbsurdExtremeComparison => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllocInsteadOfCore => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttribute => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttributeWithoutReason => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostCompleteRange => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostSwapped => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ApproxConstant => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArcWithNoSendSync => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArithmeticsSideEffects => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                },
                LintSeverity::Warn => match choose {
                    Lint::AbsolutePaths => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "\n# Why {inner} for {level} ?\n#\n# Encouraging Best Practices: By setting the lint to Warn, Clippy nudges expert users towards using relative paths whenever possible, promoting code portability and maintainability. The warning serves as a gentle reminder to consider the potential drawbacks of absolute paths.\n#\n# Flexibility with Justification: The Warn level allows expert developers to consciously choose to use absolute paths if they have a valid reason. It encourages them to think critically about their code and justify their decisions, fostering a deeper understanding of the trade-offs involved.\n#\n# Reducing Noise: Compared to the Deny level, which would prevent compilation if absolute paths are used, the Warn level is less disruptive. It allows experts to focus on other critical lints and address the AbsolutePaths warnings at their own pace, without blocking their workflow.\n#\n# Learning Opportunity: Even experienced developers can benefit from reminders about best practices. The Warn level provides an opportunity for experts to revisit their understanding of relative paths and module systems, potentially leading to improved code quality in the long run.\n#\n# Adapting to Project Needs: Different projects might have varying requirements and constraints. The Warn level offers flexibility to adapt the linting rules to the specific context of the project, allowing experts to make informed decisions based on their experience and knowledge.\n#"),
                    },
                    Lint::AbsurdExtremeComparison => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllocInsteadOfCore => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttribute => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttributeWithoutReason => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostCompleteRange => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostSwapped => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ApproxConstant => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArcWithNoSendSync => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArithmeticsSideEffects => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                },
                LintSeverity::Deny => match choose {
                    Lint::AbsolutePaths => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AbsurdExtremeComparison => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllocInsteadOfCore => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttribute => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AllowAttributeWithoutReason => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostCompleteRange => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::AlmostSwapped => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ApproxConstant => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArcWithNoSendSync => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                    Lint::ArithmeticsSideEffects => match level {
                        ClippyLevel::Novice => write!(f, "value none"),
                        ClippyLevel::Expert => write!(f, "value none"),
                        ClippyLevel::Master => write!(f, "value none"),
                    },
                },
                _ => write!(f, ""),
            },
        }
    }
}
#[doc = "All clippy lint group"]
pub enum LintGroup {
    Style,
    Correctness,
    Performance,
    Complexity,
    Pedantic,
    Restriction,
    Suspicious,
    Nursery,
    Perf,
}

impl Display for LintGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LintGroup::Style => write!(f, "style"),
            LintGroup::Correctness => write!(f, "correctness"),
            LintGroup::Performance => write!(f, "performance"),
            LintGroup::Complexity => write!(f, "complexity"),
            LintGroup::Pedantic => write!(f, "pedantic"),
            LintGroup::Restriction => write!(f, "restriction"),
            LintGroup::Suspicious => write!(f, "suspicious"),
            LintGroup::Nursery => write!(f, "nursery"),
            LintGroup::Perf => write!(f, "perf"),
        }
    }
}

#[doc = "All clippy applicability"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    Unspecified,
    Experimental,
    Stable,
    Deprecated,
    MachineApplicable,
    MaybeIncorrect,
}

impl Display for Applicability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Applicability::Unspecified => write!(f, "unspecified"),
            Applicability::Experimental => write!(f, "experimental"),
            Applicability::Stable => write!(f, "stable"),
            Applicability::Deprecated => write!(f, "deprecated"),
            Applicability::MachineApplicable => write!(f, "machine-applicable"),
            Applicability::MaybeIncorrect => write!(f, "maybe-incorrect"),
        }
    }
}
#[doc = "Represent a configuration for a lint"]
pub struct ClippyLint<'a> {
    pub id: &'static str,
    pub description: &'static str,
    pub whats_bad: &'static str,
    pub known_problems: Option<&'static str>,
    pub enabled_by_default: bool,
    pub default_clippy_severity: LintSeverity<'a>,
    pub use_clippy_severity: bool,
    pub severity: LintSeverity<'a>,
    pub group: LintGroup,
    pub url: Option<&'static str>,
    pub applicability: Applicability,
    pub all_increase_config_default_possible_severity: LintSeverity<'a>,
    pub all_decrease_config_default_possible_severity: LintSeverity<'a>,
    pub all_increase_clippy_default_possible_severity: LintSeverity<'a>,
    pub all_decrease_clippy_default_possible_severity: LintSeverity<'a>,
}
#[doc = "The clippy prefix"]
pub const ID_PREFIX: &str = "clippy::";
#[doc = "The absolute path lint id"]
pub const ABSOLUTE_PATH_ID: &str = "absolute_paths";

#[doc = "The absolute path lint description"]
pub const ABSOLUTE_PATH_DESCRIPTION: &str =
    "Checks for usage of items through absolute paths, like std::env::current_dir.";

#[doc = "The absolute path know problem"]
pub const ABSOLUTE_PATH_KNOW_PROBLEM: Option<&'static str> = Some("There are currently a few cases which are not caught by this lint:\nMacro calls. e.g. path::to::macro!()\nDerive macros. e.g. #[derive(path::to::macro)]\nAttribute macros. e.g. #[path::to::macro]");
#[doc = "The absolute path what it's bad"]
pub const ABSOLUTE_PATH_WHATS_BAD: &str = "Many codebases have their own style when it comes to importing, but one that is seldom used is using absolute paths everywhere.\n\nThis is generally considered unidiomatic, and you should add a use statement.\n\nThe default maximum segments (2) is pretty strict, you may want to increase this in clippy.toml.\nNote: One exception to this is code from macro expansion - this does not lint such cases, as using absolute paths is the proper way of referencing items in one.";
#[doc = "The absolute path url to issue"]
pub const ABSOLUTE_PATH_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+absolute_paths");

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
    url: ABSOLUTE_PATH_ISSUE,
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
    url: ABSOLUTE_PATH_ISSUE,
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
    url: ABSOLUTE_PATH_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The clippy alloc instead of core id"]
pub const ALLOC_INSTEAD_OF_CORE_ID: &str = "alloc_instead_of_core";
#[doc = "The clippy alloc instead of core description"]
pub const ALLOC_INSTEAD_OF_CORE_DESCRIPTION: &str =
    "Finds items imported through alloc when available through core.";
#[doc = "The clippy alloc instead of core know problem"]
pub const ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM: Option<&'static str> = Some("The lint is only partially aware of the required MSRV for items that were originally in std but moved to core.");
#[doc = "The clippy alloc instead of core what it's bad"]
pub const ALLOC_INSTEAD_OF_CORE_WHATS_BAD: &str = "Crates which have no_std compatibility and may optionally require alloc may wish to ensure types are imported from core to ensure disabling alloc does not cause the crate to fail to compile. This lint is also useful for crates migrating to become no_std compatible.";
#[doc = "The clippy alloc instead of core what it's bad issue url"]
pub const ALLOC_INSTEAD_OF_CORE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+alloc_instead_of_core");

#[doc = "clippy alloc instead of core lint for novice"]
pub const NOVICE_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "clippy alloc instead of core lint for expert"]
pub const EXPERT_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "clippy alloc instead of core lint for master"]
pub const MASTER_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The allow attributes id"]
pub const ALLOW_ATTRIBUTE_ID: &str = "allow_attributes";
#[doc = "The allow attributes description"]
pub const ALLOW_ATTRIBUTE_DESCRIPTION: &str = "Checks for usage of the #[allow] attribute and suggests replacing it with the #[expect]";

#[doc = "The allow attributes know problem"]
pub const ALLOW_ATTRIBUTE_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The allow attributes what it's bad"]
pub const ALLOW_ATTRIBUTE_WHATS_BAD: &str = "#[expect] attributes suppress the lint emission, but emit a warning, if the expectation is unfulfilled. This can be useful to be notified when the lint is no longer triggered.";
#[doc = "The  allow attributes issue url"]
pub const ALLOW_ATTRIBUTE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+allow_attributes");

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
    url: ALLOW_ATTRIBUTE_ISSUE,
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
    url: ALLOW_ATTRIBUTE_ISSUE,
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
    url: ALLOW_ATTRIBUTE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

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
    url: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
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
    url: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
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
    url: ALLOW_ATTRIBUTE_WITHOUT_REASON_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The almost complete range id"]
pub const ALMOST_COMPLETE_RANGE_ID: &str = "almost_complete_range";
#[doc = "The almost complete range description"]
pub const ALMOST_COMPLETE_RANGE_DESCRIPTION: &str = "Checks for ranges which almost include the entire range of letters from ‘a’ to ‘z’ or digits from ‘0’ to ‘9’, but don’t because they’re a half open range.";
#[doc = "The almost complete range know problem"]
pub const ALMOST_COMPLETE_RANGE_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The almost complete range what it's bad"]
pub const ALMOST_COMPLETE_RANGE_WHATS_BAD: &str = "This ('a'..'z') is almost certainly a typo meant to include all letters.";
#[doc = "The almost complete range issue uri"]
pub const ALMOST_COMPLETE_RANGE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+almost_complete_range");

#[doc = "The almost complete range for novice"]
pub const NOVICE_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: ALMOST_COMPLETE_RANGE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    url: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),

};

#[doc = "The almost complete range for expert"]
pub const EXPERT_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    url: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The almost complete range for master"]
pub const MASTER_ALMOST_COMPLETE_RANGE: ClippyLint = ClippyLint {
    id: ALMOST_COMPLETE_RANGE_ID,
    description: ALMOST_COMPLETE_RANGE_DESCRIPTION,
    whats_bad: ALMOST_COMPLETE_RANGE_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Suspicious,
    url: ALMOST_COMPLETE_RANGE_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The almost swapped id"]
pub const ALMOST_SWAPPED_ID: &str = "almost_swapped";
#[doc = "The almost complete range description"]
pub const ALMOST_SWAPPED_DESCRIPTION: &str = "Checks for foo = bar; bar = foo sequences.";
#[doc = "The almost swapped know problem"]
pub const ALMOST_SWAPPED_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The almost swapped what it's bad"]
pub const ALMOST_SWAPPED_WHATS_BAD: &str = "This looks like a failed attempt to swap.";
#[doc = "The almost swapped issue uri"]
pub const ALMOST_SWAPPED_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+almost_swapped");

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
    url: ALMOST_SWAPPED_ISSUE,
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
    url: ALMOST_SWAPPED_ISSUE,
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
    url: ALMOST_SWAPPED_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The approx constant id"]
pub const APPROX_CONSTANT_ID: &str = "approx_constant";
#[doc = "The approx constant description"]
pub const APPROX_CONSTANT_DESCRIPTION: &str = "Checks for floating point literals that approximate constants which are defined in std::f32::consts or std::f64::consts, respectively, suggesting to use the predefined constant.";
#[doc = "The approx constant know problem"]
pub const APPROX_CONSTANT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The approx constant what it's bad"]
pub const APPROX_CONSTANT_WHATS_BAD: &str = "Usually, the definition in the standard library is more precise than what people come up with. If you find that your definition is actually more precise.";
#[doc = "The approx constant issue uri"]
pub const APPROX_CONSTANT_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+approx_constant");

#[doc = "The almost complete range for novice"]
pub const NOVICE_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    url: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for expert"]
pub const EXPERT_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    url: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for master"]
pub const MASTER_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    url: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};
#[doc = "The arc with non send sync id"]
pub const ARC_WITH_NO_SEND_SYNC_ID: &str = "arc_with_non_send_sync";
#[doc = "The arc with non send sync description"]
pub const ARC_WITH_NO_SEND_SYNC_DESCRIPTION: &str =
    "This lint warns when you use Arc with a type that does not implement Send or Sync.";
#[doc = "The arc with non send sync know problem"]
pub const ARC_WITH_NO_SEND_SYNC_KNOW_PROBLEM: &str = "";
#[doc = "The arc with non send sync whats bad"]
pub const ARC_WITH_NO_SEND_SYNC_WHATS_BAD: &str = "Arc<T> is a thread-safe Rc<T> and guarantees that updates to the reference counter use atomic operations. To send an Arc<T> across thread boundaries and share ownership between multiple threads, T must be both Send and Sync, so either T should be made Send + Sync or a Rc should be used instead of an Arc.";
#[doc = "The arc with non send sync issue uri"]
pub const ARC_WITH_NO_SEND_SYNC_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+arc_with_non_send_sync");

#[doc = "The arc with non send sync for novice"]
pub const NOVICE_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    url: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The arc with non send sync for expert"]
pub const EXPERT_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    url: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The arc with non send sync for master"]
pub const MASTER_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    url: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The arithmetic side effects id"]
pub const ARITHMETIC_SIDE_EFFECT_ID: &str = "arithmetic_side_effects";
#[doc = "The arithmetic side effects description"]
pub const ARITHMETIC_SIDE_EFFECT_DESCRIPTION: &str = "Checks any kind of arithmetic operation of any type.";

#[doc = "The arithmetic side effects know problem"]
pub const ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The arithmetic side effects whats bad"]
pub const ARITHMETIC_SIDE_EFFECT_WHATS_BAD: &str = "For integers, overflow will trigger a panic in debug builds or wrap the result in release mode; division by zero will cause a panic in either mode. As a result, it is desirable to explicitly call checked, wrapping or saturating arithmetic methods.";
#[doc = "The arc with non send sync issue uri"]
pub const ARITHMETIC_SIDE_EFFECT_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+arithmetic_side_effects");

#[doc = "The arithmetic side effects for novice"]
pub const NOVICE_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The arithmetic side effects for expert"]
pub const EXPERT_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The arithmetic side effects for master"]
pub const MASTER_ARITHMETIC_SIDE_EFFECT: ClippyLint = ClippyLint {
    id: ARITHMETIC_SIDE_EFFECT_ID,
    description: ARITHMETIC_SIDE_EFFECT_DESCRIPTION,
    whats_bad: ARITHMETIC_SIDE_EFFECT_WHATS_BAD,
    known_problems: ARITHMETIC_SIDE_EFFECT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
#[doc = "The absurd extreme comparisons lint id"]
pub const ABSURD_EXTREME_COMPARISON_ID: &str = "absurd_extreme_comparisons";

#[doc = "The absurd extreme comparisons description"]
pub const ABSURD_EXTREME_COMPARISON_DESCRIPTION: &str =
    "Checks for usage of items through absolute paths, like std::env::current_dir.";

#[doc = "The absurd extreme comparisons know problem"]
pub const ABSURD_EXTREME_COMPARISON_KNOW_PROBLEM: Option<&'static str> = Some("For usize the size of the current compile target will be assumed (e.g., 64 bits on 64 bit systems). This means code that uses such a comparison to detect target pointer width will trigger this lint. One can use mem::sizeof and compare its value or conditional compilation attributes like #[cfg(target_pointer_width = \"64\")] .. instead.");

#[doc = "The absurd extreme comparisons what it's bad"]
pub const ABSURD_EXTREME_COMPARISON_WHATS_BAD: &str = "An expression like min <= x may misleadingly imply that it is possible for x to be less than the minimum. Expressions like max < x are probably mistakes.";

#[doc = "The absurd extreme comparisons issue url"]
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
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
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
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
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
    url: ARITHMETIC_SIDE_EFFECT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The as_conversions lint id"]
pub const AS_CONVERSIONS_ID: &str = "as_conversions";
#[doc = "The as_conversions description"]
pub const AS_CONVERSIONS_DESCRIPTION: &str = "Checks for usage of as conversions.\n#\n# Note that this lint is specialized in linting every single use of as regardless of whether good alternatives exist or not.\n#\n# If you want more precise lints for as, please consider using these separate lints: unnecessary_cast, cast_lossless/cast_possible_truncation/cast_possible_wrap/cast_precision_loss/cast_sign_loss, fn_to_numeric_cast(_with_truncation), char_lit_as_u8, ref_to_mut and ptr_as_ptr.\n#\n# There is a good explanation the reason why this lint should work in this way and how it is useful in this issue.\n#";
#[doc = "The as_conversions know problem"]
pub const AS_CONVERSIONS_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The as_conversions what it's bad"]
pub const AS_CONVERSIONS_WHATS_BAD: &str = "The as conversions will perform many kinds of conversions, including silently lossy conversions and dangerous coercions.\n#\n# There are cases when it makes sense to use as, so the lint is Allow by default.";
#[doc = "The as_conversions issue url"]
pub const AS_CONVERSIONS_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+as_conversions");

#[doc = "The as_conversions for novice"]
pub const NOVICE_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The as_conversions for expert"]
pub const EXPERT_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The as_conversions for master"]
pub const MASTER_AS_CONVERSIONS: ClippyLint = ClippyLint {
    id: AS_CONVERSIONS_ID,
    description: AS_CONVERSIONS_DESCRIPTION,
    whats_bad: AS_CONVERSIONS_WHATS_BAD,
    known_problems: AS_CONVERSIONS_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: AS_CONVERSIONS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};


#[doc = "The as underscore lint id"]
pub const AS_UNDERSCORE_ID: &str = "as_underscore";
#[doc = "The as underscore description"]
pub const AS_UNDERSCORE_DESCRIPTION: &str = "Checks for usage of as conversions.\n#\n# Note that this lint is specialized in linting every single use of as regardless of whether good alternatives exist or not.\n#\n# If you want more precise lints for as, please consider using these separate lints: unnecessary_cast, cast_lossless/cast_possible_truncation/cast_possible_wrap/cast_precision_loss/cast_sign_loss, fn_to_numeric_cast(_with_truncation), char_lit_as_u8, ref_to_mut and ptr_as_ptr.\n#\n# There is a good explanation the reason why this lint should work in this way and how it is useful in this issue.\n#";

#[doc = "The as underscore know problem"]
pub const AS_UNDERSCORE_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The as underscore what it's bad"]
pub const AS_UNDERSCORE_WHATS_BAD: &str = "The as conversions will perform many kinds of conversions, including silently lossy conversions and dangerous coercions.\n#\n# There are cases when it makes sense to use as, so the lint is Allow by default.";

#[doc = "The as underscore issue url"]
pub const AS_UNDERSCORE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+as_underscore");

#[doc = "The as_conversions for novice"]
pub const NOVICE_AS_UNDERSCORE: ClippyLint = ClippyLint {
    id: AS_UNDERSCORE_ID,
    description: AS_UNDERSCORE_DESCRIPTION,
    whats_bad: AS_UNDERSCORE_WHATS_BAD,
    known_problems: AS_UNDERSCORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: AS_UNDERSCORE_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The as underscore for expert"]
pub const EXPERT_AS_UNDERSCORE: ClippyLint = ClippyLint {
    id: AS_UNDERSCORE_ID,
    description: AS_UNDERSCORE_DESCRIPTION,
    whats_bad: AS_UNDERSCORE_WHATS_BAD,
    known_problems: AS_UNDERSCORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: AS_UNDERSCORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The as_conversions for master"]
pub const MASTER_AS_UNDERSCORE: ClippyLint = ClippyLint {
    id: AS_UNDERSCORE_ID,
    description: AS_UNDERSCORE_DESCRIPTION,
    whats_bad: AS_UNDERSCORE_WHATS_BAD,
    known_problems: AS_UNDERSCORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: AS_UNDERSCORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};


#[doc = "The assertions on result states id"]
pub const ASSERTIONS_ON_RESULTS_STATES_ID: &str = "assertions_on_result_states";
#[doc = "The assertions on result states description"]
pub const ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION: &str = "Checks for assert!(r.is_ok()) or assert!(r.is_err()) calls.";
#[doc = "The assertions on result states know problem"]
pub const ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM: Option<&'static str> = Some("The suggested replacement decreases the readability of code and log output.");
#[doc = "The assertions on result states what it's bad"]
pub const ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD: &str = "This form of assertion does not show any of the information present in the Result other than which variant it isn’t.";
#[doc = "The as underscore issue url"]
pub const ASSERTIONS_ON_RESULTS_STATES_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assertions_on_result_states");

#[doc = "The assertions on result states for novice"]
pub const NOVICE_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assertions on result states for expert"]
pub const EXPERT_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    url: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assertions on result states for master"]
pub const MASTER_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    url: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The as ptr cast mut id"]
pub const AS_PTR_CAST_MUT_ID: &str = "as_ptr_cast_mut";
#[doc = "The as ptr cast mut description"]
pub const AS_PTR_CAST_MUT_DESCRIPTION: &str = "Checks for the result of a &self-taking as_ptr being cast to a mutable pointer.";
#[doc = "The as ptr cast mut know problem"]
pub const AS_PTR_CAST_MUT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The as ptr cast mut what it's bad"]
pub const AS_PTR_CAST_MUT_WHATS_BAD: &str = "Since as_ptr takes a &self, the pointer won’t have write permissions unless interior mutability is used, making it unlikely that having it as a mutable pointer is correct.";
#[doc = "The as ptr cast mut issue url"]
pub const AS_PTR_CAST_MUT_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+as_ptr_cast_mut");

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
    url: AS_PTR_CAST_MUT_ISSUE,
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
    url: AS_PTR_CAST_MUT_ISSUE,
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
    url: AS_PTR_CAST_MUT_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assertions on constants id"]
pub const ASSERTIONS_ON_CONSTANTS_ID: &str = "assertions_on_constants";
#[doc = "The assertions on constants description"]
pub const ASSERTIONS_ON_CONSTANTS_DESCRIPTION: &str = "Checks for assert!(true) and assert!(false) calls.";
#[doc = "The assertions on constants know problem"]
pub const ASSERTIONS_ON_CONSTANTS_PROBLEM: Option<&'static str> = None;
#[doc = "The assertions on constants what it's bad"]
pub const ASSERTIONS_ON_CONSTANTS_WHATS_BAD: &str = "Will be optimized out by the compiler or should probably be replaced by a panic!() or unreachable!()";
#[doc = "The assertions on constants issue url"]
pub const ASSERTIONS_ON_CONSTANTS_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assertions_on_constants");

#[doc = "The as ptr cast mut for novice"]
pub const NOVICE_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    url: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The as ptr cast mut for expert"]
pub const EXPERT_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    url: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The as ptr cast mut for master"]
pub const MASTER_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    url: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [ClippyLint; 15] = [
    NOVICE_ABSOLUTE_PATH,
    NOVICE_ALLOC_INSTEAD_OF_CORE,
    NOVICE_ALLOW_ATTRIBUTE,
    NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    NOVICE_ALMOST_COMPLETE_RANGE,
    NOVICE_ALMOST_SWAPPED,
    NOVICE_APPROX_CONSTANT,
    NOVICE_ARC_WITH_NO_SEND_SYNC,
    NOVICE_ARITHMETIC_SIDE_EFFECT,
    NOVICE_ABSURD_EXTREME_COMPARISON,
    NOVICE_AS_CONVERSIONS,
    NOVICE_AS_UNDERSCORE,
    NOVICE_ASSERTIONS_ON_RESULTS,
    NOVICE_AS_PTR_CAST_MUT,
    NOVICE_ASSERTIONS_ON_CONSTANTS,
];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [ClippyLint; 15] = [
    EXPERT_ABSOLUTE_PATH,
    EXPERT_ALLOC_INSTEAD_OF_CORE,
    EXPERT_ALLOW_ATTRIBUTE,
    EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    EXPERT_ALMOST_COMPLETE_RANGE,
    EXPERT_ALMOST_SWAPPED,
    EXPERT_APPROX_CONSTANT,
    EXPERT_ARC_WITH_NO_SEND_SYNC,
    EXPERT_ARITHMETIC_SIDE_EFFECT,
    EXPERT_ABSURD_EXTREME_COMPARISON,
    EXPERT_AS_CONVERSIONS,
    EXPERT_AS_UNDERSCORE,
    EXPERT_ASSERTIONS_ON_RESULTS,
    EXPERT_AS_PTR_CAST_MUT,
    EXPERT_ASSERTIONS_ON_CONSTANTS,
];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [ClippyLint; 15] = [
    MASTER_ABSOLUTE_PATH,
    MASTER_ALLOC_INSTEAD_OF_CORE,
    MASTER_ALLOW_ATTRIBUTE,
    MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    MASTER_ALMOST_COMPLETE_RANGE,
    MASTER_ALMOST_SWAPPED,
    MASTER_APPROX_CONSTANT,
    MASTER_ARC_WITH_NO_SEND_SYNC,
    MASTER_ARITHMETIC_SIDE_EFFECT,
    MASTER_ABSURD_EXTREME_COMPARISON,
    MASTER_AS_CONVERSIONS,
    MASTER_AS_UNDERSCORE,
    MASTER_ASSERTIONS_ON_RESULTS,
    MASTER_AS_PTR_CAST_MUT,
    MASTER_ASSERTIONS_ON_CONSTANTS
];

#[doc = "Novice configuration filename"]
pub const NOVICE_CONFIGURATION_FILENAME: &str = "novice.toml";

#[doc = "Expert configuration filename"]
pub const EXPERT_CONFIGURATION_FILENAME: &str = "expert.toml";

#[doc = "Master configuration filename"]
pub const MASTER_CONFIGURATION_FILENAME: &str = "master.toml";

#[doc = "Legendary configuration filename"]
pub const LEGENDARY_CONFIGURATION_FILENAME: &str = "legendary.toml";

#[doc = "configuration storage directory"]
pub const CONFIG_DIRECTORY: &str = "config";

#[doc = "Generates configuration files based on skill levels"]
pub fn generate_config() -> Result<(), std::io::Error> {
    create_dir_all(CONFIG_DIRECTORY)?;

    let novice_lints = NOVICE_LINTS;
    let expert_lints = EXPERT_LINTS;
    let master_lints = MASTER_LINTS;
    generate_config_file(NOVICE_CONFIGURATION_FILENAME, &novice_lints)?;
    generate_config_file(EXPERT_CONFIGURATION_FILENAME, &expert_lints)?;
    generate_config_file(MASTER_CONFIGURATION_FILENAME, &master_lints)?;

    Ok(())
}
#[doc = "Generates a single configuration file."]
pub fn generate_config_file(filename: &str, lints: &[ClippyLint]) -> Result<(), std::io::Error> {
    let file_path = Path::new(CONFIG_DIRECTORY).join(filename);
    let mut file = File::create(file_path)?;

    for lint in lints {
        writeln!(
            file,
            "#\n# Lint {ID_PREFIX}{}\n#\n# {}\n#\n# {}",
            &lint.id,
            &lint.description,
            &lint.whats_bad.replace("\n", "\n# ")
        )?;
        if let Some(uri) = lint.url {
            writeln!(file, "#\n# Issue url : {}\n#", uri)?;
        }
        writeln!(
            file,
            "# Clippy decrease possible {}\n#\n# Clippy increase possible {}\n#\n# Default configuration decrease possible {}\n#\n# Default configuration increase possible {}\n#",
            lint.all_decrease_clippy_default_possible_severity,
            lint.all_increase_clippy_default_possible_severity,
            lint.all_decrease_config_default_possible_severity,
            lint.all_increase_config_default_possible_severity,
        )?;
        writeln!(file, "[{}]", lint.id)?;
        writeln!(file, "group = \"{}\"", lint.group)?;
        writeln!(file, "applicability = \"{}\"", lint.applicability)?;
        writeln!(file, "enabled = {}", lint.enabled_by_default)?;
        writeln!(file, "config-severity = \"{}\"", lint.severity)?;
        writeln!(file, "clippy-severity = \"{}\"", lint.default_clippy_severity)?;
        writeln!(file, "use-clippy-severity = {}\n", lint.use_clippy_severity)?;
    }
    Ok(())
}