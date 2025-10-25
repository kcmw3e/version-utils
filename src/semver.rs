// Semantic versioning module to handle parsing and storage of version
// information.
//------------------------------------------------------------------------------

use log;
use std::str::FromStr;

#[derive(Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
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
