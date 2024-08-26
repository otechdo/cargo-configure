//!
//! # Cargo configure
//!
//! It's a tools to configure new depot in order to simplify adn personalize my zuu crates and clippy more simply.
//!
//!
use crate::config::generate::configure;
use std::io::Error;
#[doc = "All cargo-configure clippy lints configuration"]
pub mod clippy;
#[doc = "All cargo-configure configuration"]
pub mod config;
#[doc = "All cargo-configure available lints"]
pub mod lints;

#[doc = "Configure a cargo project"]
pub fn main() -> Result<(), Error> {
    configure()
}
