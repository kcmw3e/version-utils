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

/// Parse a number from the string up to a dot (.) character.
///
/// The remaining part of the string (without the dot) is returned along with
/// the parsed number, unless there was an error during parsing.
fn parse_number_to_dot<T>(string: &str) -> Result<(T, &str), ParseError>
where
    T: FromStr,
{
    log::trace!("Parsing number from {string:?} until dot.");

    let Some((maybe_number, rest)) = string.split_once(".") else {
        log::error!("Could not find dot separator between version parts.");
        return Err(ParseError::NoSeparatorFound);
    };

    log::trace!(
        "Parsed number into ({maybe_number:?}, {rest:?}) at dot separator."
    );

    let number: T = parse_number(maybe_number)?;

    Ok((number, rest))
}
