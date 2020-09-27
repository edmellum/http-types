use crate::{bail, Body, Mime};

use std::fmt::{self, Debug};
use std::path::Path;

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

    /// Create an empty `Entry`.
    pub fn empty(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            body: Body::empty(),
        }
    }

    /// Create an `Entry` from a file.
    ///
    #[cfg(all(feature = "async_std", not(target_os = "unknown")))]
    pub async fn from_file<P>(path: P) -> crate::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        let name = match path.to_str() {
            Some(p) => p.to_owned(),
            None => bail!("Could not convert file name to unicode"),
        };
        let body = Body::from_file(path).await?;
        Ok(Self::new(name, body))
    }

    /// Get the entry name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set the entry name.
    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.name = name.as_ref().to_owned();
    }

    /// Get the content type.
    pub fn content_type(&self) -> Option<Mime> {
        todo!();
    }

    /// Set the content type.
    pub fn set_content_type(&mut self, _mime: Option<Mime>) {
        todo!();
    }

    /// Get the file name of the entry, if it's set.
    pub fn file_name(&self) -> Option<&Path> {
        self.body.file_name().map(|p| p.as_path())
    }

    /// Set the file name of the `Body`.
    pub fn set_file_name<P>(&mut self, file_name: Option<P>)
    where
        P: AsRef<Path>,
    {
        self.body.set_file_name(file_name);
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
