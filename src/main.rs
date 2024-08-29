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

use inquire::MultiSelect;

const CLIPPY_GROUPS: [&str; 9] = [
    "cargo",
    "complexity",
    "restriction",
    "style",
    "nursery",
    "pedantic",
    "suspicious",
    "correctness",
    "perf",
];

fn decrease(g: &mut Vec<String>, data: Vec<String>) {
    for d in &data {
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
        .with_default(&[0, 2, 4, 5])
        .prompt()
        .unwrap_or_default();

    decrease(&mut groups, allowed.to_vec());

    let warn = MultiSelect::new("Select the warning groups : ", groups.clone())
        .prompt()
        .unwrap_or(groups.to_vec());

    decrease(&mut groups, warn.to_vec());

    write!(
        zuu,
        "allow = {allowed:?}\nwarn = {warn:?}\ndeny = {groups:?}"
    )
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
    generate_deny()
}
///
/// Configure a project
pub fn main() -> Result<(), Error> {
    configure()
}
