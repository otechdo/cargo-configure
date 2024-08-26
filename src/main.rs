use std::io::Error;
use crate::config::generate::generate_config;
#[doc = "All cargo-configure configuration"]
pub mod config;
#[doc = "All cargo-configure clippy lints configuration"]
pub mod clippy;
#[doc = "All cargo-configure available lints"]
pub mod lints;

pub fn main() -> Result<(), Error> {
    generate_config()
}