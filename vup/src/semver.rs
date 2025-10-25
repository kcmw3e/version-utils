// Semantic versioning module to handle parsing and storage of version
// information.
//------------------------------------------------------------------------------

use log;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// A specification for version bumps.
///
/// Version bumps are done this way instead of through separate functions for a
/// few reasons. One is so that all of the bumping logic is in the same place,
/// which makes future additions to it the bump context easier (such as allowing
/// the patch component to stay consistent during a bump, for example).
///
/// Doing it this way also allows for chaining bumps easily, especially if a
/// series of various bump levels are needed. For example:
///
/// ```rs
/// # use vup::semver::{Bump, Version};
///
/// let v = Version{major: 1, minor: 2, patch: 3};
/// let bumps = [Bump::Major(1), Bump::Minor(3), Bump::Patch(4)];
///
/// assert_eq!(
///     bumps.iter().fold(v, |nv, bump| nv.bump(*bump)),
///     Version{major: 2, minor: 3, patch: 4},
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Bump {
    Major(u64),
    Minor(u64),
    Patch(u64),
}

impl Version {
    const CORE_VERSION_SEPARATOR: char = '.';

    pub fn bump(&self, bumpspec: Bump) -> Self {
        match bumpspec {
            Bump::Major(diff) => {
                Self { major: self.major + diff, minor: 0, patch: 0 }
            },
            Bump::Minor(diff) => {
                Self { minor: self.minor + diff, patch: 0, ..*self }
            },
            Bump::Patch(diff) => Self { patch: self.patch + diff, ..*self },
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    /// No separator between version parts was found when one was expected. For
    /// the core version, this is a dot (.) character.
    #[error("separator '{separator}' could not be fouund")]
    NoSeparatorFound { separator: char },
    /// An invalid character was found when trying to parse a number.
    #[error("could not parse number (non-digit character found)")]
    InvalidNumber,
}

/// The various parsing states and the contexts required for parsing them.
///
/// The `string` field in each state is the leftover un-parsed string from the
/// previous state. Each enumeration acts as the state context and is
/// essentially the partially-filled version up to that point. Thus, each state
/// encapsulates the prior state's already-known version parts.
enum ParseState<'parser> {
    Trim { string: &'parser str },
    Major { string: &'parser str },
    Minor { string: &'parser str, major: u64 },
    Patch { string: &'parser str, major: u64, minor: u64 },
    Finished { major: u64, minor: u64, patch: u64 },
}

/// Parse a version string into a semantic version according to the
/// specification outlined at https://semver.org/
///
/// The string may contain whitespaces only at the beginning and end (which will
/// immediately be discarded).
///
/// Note that only the major, minor, and patch components of a semantic version
/// are currently supported; thus the string must be in the format
/// `<MAJOR>.<MINOR>.<PATCH>` (where `MAJOR`, `MINOR`, and `PATCH` are all
/// solely comprised of digits), and will otherwise fail to parse.
pub fn parse(string: &str) -> Result<Version, ParseError> {
    log::trace!("Parsing semantic version from {string:?}.");

    let mut parse_state = ParseState::Trim { string };

    loop {
        match parse_state {
            ParseState::Trim { string } => {
                parse_state = ParseState::Major { string: string.trim() }
            },
            ParseState::Major { string } => {
                let (major, rest) = parse_number_to_separator(
                    string,
                    Version::CORE_VERSION_SEPARATOR,
                )?;
                parse_state = ParseState::Minor { string: rest, major };
            },
            ParseState::Minor { string, major } => {
                let (minor, rest) = parse_number_to_separator(
                    string,
                    Version::CORE_VERSION_SEPARATOR,
                )?;
                parse_state = ParseState::Patch { string: rest, major, minor };
            },
            ParseState::Patch { string, major, minor } => {
                let patch = parse_number(string)?;
                parse_state = ParseState::Finished { major, minor, patch };
            },
            ParseState::Finished { major, minor, patch } => {
                return Ok(Version { major, minor, patch });
            },
        }
    }
}

/// Parse a number from a string.
///
/// This is just a wrapper around the builtin `parse` method of `str` except it
/// adds some logging and error information.
fn parse_number<T>(string: &str) -> Result<T, ParseError>
where
    T: FromStr,
{
    log::trace!("Parsing number from {string:?}.");

    let Ok(number) = string.parse::<T>() else {
        log::error!("Could not parse number from {string:?}.");
        return Err(ParseError::InvalidNumber);
    };

    Ok(number)
}

/// Parse a number from the string up to the separator character.
///
/// The remaining part of the string (without the separator) is returned along
/// with the parsed number, unless there was an error during parsing.
fn parse_number_to_separator<T>(
    string: &str,
    separator: char,
) -> Result<(T, &str), ParseError>
where
    T: FromStr,
{
    log::trace!("Parsing number from {string:?} until dot.");

    let Some((maybe_number, rest)) = string.split_once(separator) else {
        log::error!("Could not find dot separator between version parts.");
        return Err(ParseError::NoSeparatorFound { separator });
    };

    log::trace!(
        "Parsed number into ({maybe_number:?}, {rest:?}) at dot separator."
    );

    let number: T = parse_number(maybe_number)?;

    Ok((number, rest))
}
