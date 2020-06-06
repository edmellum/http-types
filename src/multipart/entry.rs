use crate::Body;

use std::fmt::{self, Debug};
use std::path::PathBuf;

/// A single multipart entry.
///
/// Structurally Multipart entries are similar to `Body`.
pub struct Entry {
    name: String,
    body: Body,
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new(name: impl AsRef<str>, body: impl Into<Body>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            body: body.into(),
        }
    }

    /// Get the entry name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the file name of the entry, if it's set.
    pub fn file_name(&self) -> Option<&PathBuf> {
        self.body.file_name.as_ref()
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("name", &self.name)
            .field("body", &self.body)
            .finish()
    }
}

// TODO
// impl AsyncRead for Entry {}
// impl AsRef<Body> for Entry {}
// impl AsMut<Body> for Entry {}
// impl Into<Body> for Entry {}
