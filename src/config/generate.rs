use crate::config::{Lint, DOC_BASE_LINK, ISSUE_BASE_LINK};
use crate::lints::{EXPERT_LINTS, MASTER_LINTS, NOVICE_LINTS};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

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

///
///
/// # Errors
///
/// Exit failure without write mode on the current directory
///
#[doc = "Generates configuration files based on skill levels"]
pub fn configure() -> Result<(), std::io::Error> {
    create_dir_all(CONFIG_DIRECTORY)?;
    let mut novice_lints = NOVICE_LINTS;
    let mut expert_lints = EXPERT_LINTS;
    let mut master_lints = MASTER_LINTS;
    generate(NOVICE_CONFIGURATION_FILENAME, &mut novice_lints)?;
    generate(EXPERT_CONFIGURATION_FILENAME, &mut expert_lints)?;
    generate(MASTER_CONFIGURATION_FILENAME, &mut master_lints)?;

    Ok(())
}

///
///
/// # Errors
///
/// Exit failure without write mode on the current directory
///
#[doc = "Generates a single configuration file."]
pub fn generate(filename: &str, lints: &mut [Lint]) -> Result<(), std::io::Error> {
    let file_path = Path::new(CONFIG_DIRECTORY).join(filename);
    let mut file = File::create(file_path)?;
    lints.sort();
    for lint in lints {
        writeln!(file, "# {}", lint.description)?;
        writeln!(file, "[{}]", lint.id)?;
        writeln!(file, "group = \"{}\"", lint.group)?;
        writeln!(file, "applicability = \"{}\"", lint.applicability)?;
        writeln!(
            file,
            "# {} < {} > {}\nseverity = \"{}\"",
            lint.decrease, lint.severity, lint.increase, lint.severity
        )?;
        writeln!(file, "issue = \"{ISSUE_BASE_LINK}+{}\"", lint.id)?;
        writeln!(file, "info = \"{DOC_BASE_LINK}/{}\"", lint.id)?;
    }
    Ok(())
}
