use crate::config::{Applicability, ClippyGroup, Lint, Severity};

#[doc = "The lints for all"]
pub const ABSOLUTE_PATH_LINTS: (Lint, Lint, Lint) = Lint::new(
    "absolute_paths",
    "Checks for usage of items through absolute paths, like std::env::current_dir.",
    &Severity::Allow,
    &Severity::Warn,
    &Severity::Deny,
    ClippyGroup::Restriction,
    Applicability::Unspecified,
);
