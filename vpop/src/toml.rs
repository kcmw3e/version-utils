// Module for reading and writing version numbers in TOML files.
//------------------------------------------------------------------------------

use std::io::Read;
use thiserror::Error;
use toml_edit::{DocumentMut, Item};
use vup::semver;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("empty TOML path")]
    EmptyPath,
    #[error("missing TOML key '{0}'")]
    MissingKey(String),
    // TODO: Also provide more info here like the key tried and type found.
    #[error("entry is the wrong TOML type")]
    WrongType,
    #[error("could not parse version: {0}")]
    VersionParseError(#[from] semver::ParseError),
    #[error("could not handle TOML: {0}")]
    Toml(#[from] toml_edit::TomlError),
}

/// Get the version number from the TOML document given the specified path.
///
/// The path `path` should be given as a string with dot-separated keys. This
/// particular implementation is limited to very simple key paths. Keys must
/// not be quoted, and array indexing is not possible.
///
/// As an example, the key `"my-project.version"` is valid, which means the
/// value associated with key `"version"` in the table `"my-project"` in the
/// root table. The key `"my-project.'first-version'"` is not valid for this
/// function despite being a viable TOML key.
pub fn get_version<T>(
    doc: DocumentMut,
    path: &str,
) -> Result<semver::Version, ParseError>
where
    T: Read,
{
    let item = path.split(".").try_fold(
        doc.as_item(),
        |item, key| -> Result<&Item, ParseError> {
            // TODO: can we get better formatting?
            Ok(item
                .as_table()
                .ok_or(ParseError::WrongType)?
                .get(key)
                .ok_or(ParseError::MissingKey(String::from(key)))?)
        },
    )?;

    Ok(semver::parse(item.as_str().ok_or(ParseError::WrongType)?)?)
}
