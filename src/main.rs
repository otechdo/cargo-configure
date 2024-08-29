#![allow(clippy::multiple_crate_versions)]
//!
//! # Cargo configure
//!
//! It's a tools to configure new depot in order to simplify adn personalize my zuu crates and clippy more simply.
//!
//!
use std::{
    fs::{remove_file, File},
    io::{Error, ErrorKind, Write},
    path::Path,
    process::Command,
};

use inquire::{Confirm, MultiSelect, Select};

const CLIPPY_GROUPS: [&str; 8] = [
    "cargo",
    "complexity",
    "style",
    "nursery",
    "pedantic",
    "suspicious",
    "correctness",
    "perf",
];

const HOOKS: [&str; 7] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "test -j 4 --no-fail-fast -- --show-output",
    "fmt --check",
    "outdated",
];

fn decrease(g: &mut Vec<String>, data: &[String]) {
    for d in data {
        g.retain(|x| !x.eq(d));
    }
}
fn generate_zuu() -> Result<(), Error> {
    if Path::new("zuu.toml").exists() {
        remove_file("zuu.toml")?;
    }
    let mut zuu: File = File::create_new("zuu.toml")?;

    let mut groups: Vec<String> = CLIPPY_GROUPS.map(String::from).to_vec();
    let allowed = MultiSelect::new("Select the allowed groups : ", groups.clone())
        .prompt()
        .unwrap_or_else(|_| Vec::from(["cargo".to_string(), "pedantic".to_string()]));

    decrease(&mut groups, &allowed.clone());

    let warn = MultiSelect::new("Select the warning groups : ", groups.clone())
        .prompt()
        .unwrap_or_else(|_| groups.clone());

    decrease(&mut groups, &warn.clone());

    assert!(write!(
        zuu,
        "allow = {allowed:?}\nwarn = {warn:?}\nforbid = {groups:?}\nbefore-cargo = []\ncargo = {HOOKS:?}\nafter-cargo = []"
    )
    .is_ok());
    Ok(())
}

fn generate_deny() -> Result<(), Error> {
    if Path::new("deny.toml").exists() {
        remove_file("deny.toml")?;
    }

    if let Ok(mut child) = Command::new("cargo")
        .arg("deny")
        .arg("init")
        .current_dir(".")
        .spawn()
    {
        assert!(child.wait()?.success());
        return Ok(());
    }
    Err(Error::new(ErrorKind::NotFound, "missing cargo-deny"))
}
fn configure_zuu() -> Result<(), Error> {
    generate_zuu()
}
fn configure() -> Result<(), Error> {
    assert!(configure_zuu().is_ok());
    assert!(create_project().is_ok());
    generate_deny()
}

fn create_project() -> Result<(), Error> {
    if Path::new("Cargo.toml").exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Cargo project already exist",
        ));
    }
    let vcs: &str = Select::new(
        "Enter the versionning file systems for your project : ",
        vec!["git", "hg", "pijul", "fossil", "none"],
    )
    .prompt()
    .unwrap_or("git");
    let program = Confirm::new("Create a a new software ? : ")
        .with_default(true)
        .prompt()
        .unwrap_or_default();
    if program {
        if let Ok(mut child) = Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .arg("--vcs")
            .arg(vcs)
            .current_dir(".")
            .spawn()
        {
            assert!(child.wait()?.success());
            return Ok(());
        }
        return Err(Error::last_os_error());
    }
    if let Ok(mut child) = Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg("--vcs")
        .arg(vcs)
        .current_dir(".")
        .spawn()
    {
        assert!(child.wait()?.success(), "failed");
        return Ok(());
    }
    Err(Error::last_os_error())
}
///
/// Configure a project
pub fn main() -> Result<(), Error> {
    configure()
}
