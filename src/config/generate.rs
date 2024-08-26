use crate::config::ClippyLint;
use crate::lints::{EXPERT_LINTS, MASTER_LINTS, NOVICE_LINTS};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

#[doc = "clippy prefix"]
pub const ID_PREFIX: &str = "clippy::";

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
    let novice_lints = NOVICE_LINTS;
    let expert_lints = EXPERT_LINTS;
    let master_lints = MASTER_LINTS;
    generate(NOVICE_CONFIGURATION_FILENAME, &novice_lints)?;
    generate(EXPERT_CONFIGURATION_FILENAME, &expert_lints)?;
    generate(MASTER_CONFIGURATION_FILENAME, &master_lints)?;

    Ok(())
}

///
///
/// # Errors
///
/// Exit failure without write mode on the current directory
///
#[doc = "Generates a single configuration file."]
pub fn generate(filename: &str, lints: &[ClippyLint]) -> Result<(), std::io::Error> {
    let file_path = Path::new(CONFIG_DIRECTORY).join(filename);
    let mut file = File::create(file_path)?;

    for lint in lints {
        writeln!(
            file,
            "#\n# Lint {ID_PREFIX}{}\n#\n# {}\n#\n# {}",
            &lint.id,
            &lint.description,
            &lint.whats_bad.replace('\n', "\n# ")
        )?;
        if let Some(uri) = lint.issue {
            writeln!(file, "#\n# Issue : {uri}\n#")?;
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
        writeln!(
            file,
            "clippy-severity = \"{}\"",
            lint.default_clippy_severity
        )?;
        writeln!(file, "use-clippy-severity = {}\n", lint.use_clippy_severity)?;
    }
    Ok(())
}
