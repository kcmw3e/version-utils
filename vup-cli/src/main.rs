// A command line utility to help with bumping version numbers.
//------------------------------------------------------------------------------

use std::io::{IsTerminal, Write, stdin, stdout};

use thiserror::Error;

use clap::{Parser, Subcommand};

use vup::semver;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long)]
    only: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Major,
    Minor,
    Patch,
}

#[derive(Error, Debug)]
pub enum VupError {
    #[error("encountered an error parsing the version: {0}")]
    ParseError(#[from] semver::ParseError),
    #[error("could not perform IO: {0}")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), VupError> {
    env_logger::init();

    let stdin = stdin();
    let mut stdout = stdout();

    let cli = Cli::parse();

    let bump = match &cli.command {
        Commands::Major => semver::Bump::Major(1),
        Commands::Minor => semver::Bump::Minor(1),
        Commands::Patch => semver::Bump::Patch(1),
    };

    let mut version_string = String::new();

    stdin.read_line(&mut version_string)?;
    log::info!("Read version from stdin: '{version_string}'");

    let version = semver::parse(version_string.as_str())?;
    log::info!("Parsed version as {:?}", version);

    let semver::Version { major, minor, patch } = version.bump(bump);

    write!(stdout, "{major}.{minor}.{patch}")?;

    if stdout.is_terminal() {
        write!(stdout, "\n")?;
    }

    Ok(())
}
