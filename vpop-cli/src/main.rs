// A command line utility to retrieve and set version numbers in formatted text.
//------------------------------------------------------------------------------

use std::io::{IsTerminal, Read, Write, stdin, stdout};

use thiserror::Error;

use clap::Parser;
use toml_edit::DocumentMut;
use vpop::toml;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, short)]
    path: String,
}

#[derive(Error, Debug)]
pub enum VpopError {
    #[error("encountered an error parsing the version: {0}")]
    ParseError(#[from] toml::ParseError),
    #[error("error parsing TOML: {0}")]
    TomlError(#[from] toml_edit::TomlError),
    #[error("could not perform IO: {0}")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), VpopError> {
    env_logger::init();

    let mut stdin = stdin();
    let mut stdout = stdout();

    let cli = Cli::parse();

    let mut contents = String::new();
    stdin.read_to_string(&mut contents)?;

    let doc = contents.parse::<DocumentMut>()?;

    let version = toml::get_version(doc, cli.path.as_str())?;

    log::debug!("Found version as '{:?}'", version);

    write!(stdout, "{}.{}.{}", version.major, version.minor, version.patch)?;

    if stdout.is_terminal() {
        write!(stdout, "\n")?;
    }

    Ok(())
}
