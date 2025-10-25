// Semantic versioning module to handle parsing and storage of version
// information.
//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

