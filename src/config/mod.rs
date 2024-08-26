#[doc = "The generate configuration"]
pub mod generate;

use std::fmt::{Display, Formatter};

#[doc = "The clippy prefix"]
pub const ID_PREFIX: &str = "clippy::";

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
