use std::fmt::{Display, Formatter};

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
    pub issue: Option<&'static str>,
    pub applicability: Applicability,
    pub all_increase_config_default_possible_severity: LintSeverity<'a>,
    pub all_decrease_config_default_possible_severity: LintSeverity<'a>,
    pub all_increase_clippy_default_possible_severity: LintSeverity<'a>,
    pub all_decrease_clippy_default_possible_severity: LintSeverity<'a>,
}
#[doc = "The clippy prefix"]
pub const ID_PREFIX: &str = "clippy::";