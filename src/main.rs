// A command line utility to help with bumping version numbers.
//------------------------------------------------------------------------------

use std::io::{IsTerminal, Write, stdin, stdout};

use semver::ParseError;
use thiserror::Error;

mod semver;

#[derive(Error, Debug)]
pub enum VupError {
    #[error("encountered an error parsing the version: {0}")]
    ParseError(#[from] ParseError),
    #[error("could not perform IO: {0}")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), VupError> {
    env_logger::init();

    let stdin = stdin();
    let mut stdout = stdout();

    let mut version_string = String::new();

    stdin.read_line(&mut version_string)?;
    log::info!("Read version from stdin: '{version_string}'");

    let version = semver::parse(version_string.as_str())?;
    log::info!("Parsed version as {:?}", version);

    let semver::Version { major, minor, patch } = version;

    write!(stdout, "{major}.{minor}.{patch}")?;

    if stdout.is_terminal() {
        write!(stdout, "\n")?;
    }

    Ok(())
}
