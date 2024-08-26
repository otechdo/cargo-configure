#[doc = "The generate configuration"]
pub mod generate;
use std::fmt::{Display, Formatter};

#[doc = "The clippy prefix"]
pub const ID_PREFIX: &str = "clippy::";

#[doc = "All developer level"]
#[derive(Copy, Clone)]
pub enum ClippyLevel {
    #[doc = "It focuses on essential lints that catch common mistakes and promote basic best practice"]
    Novice,
    #[doc = "It includes a broader set of lints, covering style, performance, and potential correctness issues"]
    Expert,
    #[doc = "It focus on enforcing strict coding practices and ensuring optimal code performance and correctness high level"]
    Master,
}
impl Display for ClippyLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Novice => write!(f, "Novice"),
            Self::Expert => write!(f, "Expert"),
            Self::Master => write!(f, "Master"),
        }
    }
}
#[doc = "All severity level"]
#[derive(Copy, Clone)]
pub enum LintSeverity<'a> {
    #[doc = "Set clippy lint to allow"]
    Allow,
    #[doc = "Set clippy lint to warn"]
    Warn,
    #[doc = "Set clippy lint to deny"]
    Deny,
    #[doc = "Display all clippy possible increase values"]
    Increase(&'a Self),
    #[doc = "Display all clippy possible decrease values"]
    Decrease(&'a Self),
}

impl<'a> Display for LintSeverity<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Warn => write!(f, "warn"),
            Self::Deny => write!(f, "deny"),
            Self::Increase(inner) => match inner {
                Self::Allow => write!(f, "values : (warn, deny)"),
                Self::Deny => write!(f, "value none"),
                Self::Warn | LintSeverity::Increase(_) | LintSeverity::Decrease(_) => {
                    write!(f, "value : deny")
                }
            },
            LintSeverity::Decrease(inner) => match inner {
                Self::Allow => write!(f, "value none"),
                Self::Deny => write!(f, "values : (allow, warn)"),
                Self::Warn | Self::Increase(_) | Self::Decrease(_) => writeln!(f, "value : allow"),
            },
        }
    }
}

#[doc = "All clippy lint group"]
#[derive(Copy, Clone)]
pub enum LintGroup {
    #[doc = "Lints related to code style and formatting."]
    Style,
    #[doc = "Lints that detect potential correctness issues, such as logic errors or incorrect usage of language features."]
    Correctness,
    #[doc = "Lints that suggest ways to improve code performance."]
    Performance,
    #[doc = "Lints that flag code that might be unnecessarily complex or difficult to understand."]
    Complexity,
    #[doc = " Lints that enforce strict coding standards and stylistic conventions."]
    Pedantic,
    #[doc = "Lints that restrict certain language features or patterns that might be error-prone or lead to unexpected behavior."]
    Restriction,
    #[doc = " Lints that highlight code that looks suspicious or might indicate a potential error."]
    Suspicious,
    #[doc = "Lints that are still under development or might produce more false positives."]
    Nursery,
    #[doc = " Lints that specifically focus on performance optimizations."]
    Perf,
}

impl Display for LintGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Style => write!(f, "style"),
            Self::Correctness => write!(f, "correctness"),
            Self::Performance => write!(f, "performance"),
            Self::Complexity => write!(f, "complexity"),
            Self::Pedantic => write!(f, "pedantic"),
            Self::Restriction => write!(f, "restriction"),
            Self::Suspicious => write!(f, "suspicious"),
            Self::Nursery => write!(f, "nursery"),
            Self::Perf => write!(f, "perf"),
        }
    }
}

#[doc = "All clippy applicability"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    #[doc = "The applicability of the lint is not fully specified or understood."]
    Unspecified,
    #[doc = "The lint is experimental and might produce false positives or have other issues. of the lint is not fully specified or understood."]
    Experimental,
    #[doc = "The lint is considered stable and reliable."]
    Stable,
    #[doc = "The lint is deprecated and will be removed in a future version of Clippy."]
    Deprecated,
    #[doc = "The lint can be automatically applied by tools without requiring manual review."]
    MachineApplicable,
    #[doc = "The lint might indicate a potential correctness issue in the code."]
    MaybeIncorrect,
}

impl Display for Applicability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified"),
            Self::Experimental => write!(f, "experimental"),
            Self::Stable => write!(f, "stable"),
            Self::Deprecated => write!(f, "deprecated"),
            Self::MachineApplicable => write!(f, "machine-applicable"),
            Self::MaybeIncorrect => write!(f, "maybe-incorrect"),
        }
    }
}

#[doc = "Represent a configuration for a lint"]
pub struct ClippyLint<'a> {
    #[doc = "The lint id"]
    pub id: &'static str,
    #[doc = "The lint description"]
    pub description: &'static str,
    #[doc = "The lint explication what is bad"]
    pub whats_bad: &'static str,
    #[doc = "The lint know problem"]
    pub known_problems: Option<&'static str>,
    #[doc = "Set config lint enabled or disabled"]
    pub enabled_by_default: bool,
    #[doc = "Set lint severity can be allow, warn, deny based on clippy"]
    pub default_clippy_severity: LintSeverity<'a>,
    #[doc = "Define the config policy for allow, warn, deny severity"]
    pub use_clippy_severity: bool,
    #[doc = "Set the config severity to allow or warn or deny"]
    pub severity: LintSeverity<'a>,
    #[doc = "Set the lint group"]
    pub group: LintGroup,
    #[doc = "Set the lint issue uri"]
    pub issue: Option<&'static str>,
    #[doc = "Set the applicability value"]
    pub applicability: Applicability,
    #[doc = "Set all values possible to increase the default config value"]
    pub all_increase_config_default_possible_severity: LintSeverity<'a>,
    #[doc = "Set all values possible to decrease the default config value"]
    pub all_decrease_config_default_possible_severity: LintSeverity<'a>,
    #[doc = "Set all values possible to increase the default clippy value"]
    pub all_increase_clippy_default_possible_severity: LintSeverity<'a>,
    #[doc = "Set all values possible to decrease the default clippy value"]
    pub all_decrease_clippy_default_possible_severity: LintSeverity<'a>,
}
