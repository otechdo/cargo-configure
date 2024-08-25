use crate::clippy::generate_config;
use std::io::Error;
pub mod clippy;

fn main() -> Result<(), Error> {
    generate_config()
}
