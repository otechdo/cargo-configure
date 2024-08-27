#[doc = "The generate configuration"]
pub mod generate;
use std::fmt::{Display, Formatter};

#[doc = "the base uri for doc link"]
pub const DOC_BASE_LINK: &str = "https://rust-lang.github.io/rust-clippy/master/index.html#";

#[doc = "The base uri for issue link"]
pub const ISSUE_BASE_LINK: &str = "https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue";

#[doc = "All severity level"]
#[derive(Copy, Clone)]
pub enum Severity<'a> {
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

impl<'a> Display for Severity<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::Warn => write!(f, "warn"),
            Self::Deny => write!(f, "deny"),
            Self::Increase(inner) => match inner {
                Self::Allow => write!(f, "warn deny"),
                Self::Deny => write!(f, "none"),
                Self::Warn | Self::Increase(_) | Self::Decrease(_) => {
                    write!(f, "deny")
                }
            },
            Self::Decrease(inner) => match inner {
                Self::Allow => write!(f, "none"),
                Self::Deny => write!(f, "allow warn"),
                Self::Warn | Self::Increase(_) | Self::Decrease(_) => write!(f, "allow"),
            },
        }
    }
}

#[doc = "All clippy lint group"]
#[derive(Copy, Clone)]
pub enum ClippyGroup {
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

impl Display for ClippyGroup {
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
    #[doc = "The suggestion cannot be applied automatically because it will not result in valid Rust code. The user will need to fill in the placeholders."]
    HasPlaceholders,
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
            Self::HasPlaceholders => write!(f, "has-placeholders"),
        }
    }
}

#[doc = "Represent a configuration for a lint by generate tool"]
pub struct Lint<'a> {
    #[doc = "The lint id"]
    pub id: &'static str,
    #[doc = "The lint description"]
    pub description: &'static str,
    #[doc = "Set lint severity can be allow, warn, deny based on clippy"]
    pub severity: Severity<'a>,
    #[doc = "Set the lint group"]
    pub group: ClippyGroup,
    #[doc = "Set the applicability value"]
    pub applicability: Applicability,
    #[doc = "Set all values possible to increase the severity value"]
    pub increase: Severity<'a>,
    #[doc = "Set all values possible to decrease the severity value"]
    pub decrease: Severity<'a>,
}

impl<'a> Lint<'a> {
    #[must_use]
    #[doc = "generate lints"]
    pub const fn new(
        id: &'static str,
        description: &'static str,
        novice: &'a Severity<'a>,
        expert: &'a Severity<'a>,
        master: &'a Severity<'a>,
        group: ClippyGroup,
        applicability: Applicability,
    ) -> (Self, Self, Self) {
        (
            Self {
                id,
                description,
                severity: *novice,
                group,
                applicability,
                increase: Severity::Increase(novice),
                decrease: Severity::Decrease(novice),
            },
            Self {
                id,
                description,
                severity: *expert,
                group,
                applicability,
                increase: Severity::Increase(expert),
                decrease: Severity::Decrease(expert),
            },
            Self {
                id,
                description,
                severity: *master,
                group,
                applicability,
                increase: Severity::Increase(master),
                decrease: Severity::Decrease(master),
            },
        )
    }
}
