use crate::{Body, Mime};

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
    pub fn new<S, B>(name: S, body: B) -> Self
    where
        S: AsRef<str>,
        B: Into<Body>,
    {
        Self {
            name: name.as_ref().to_owned(),
            body: body.into(),
        }
    }

    /// Create an empty `Entry`.
    pub fn empty<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::new(name, Body::empty())
    }

    /// Create an `Entry` from a file.
    #[cfg(all(feature = "async_std", not(target_os = "unknown")))]
    pub async fn from_file<S, P>(name: S, path: P) -> crate::Result<Self>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
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
    pub fn file_name(&self) -> Option<&str> {
        self.body.file_name()
    }

    /// Set the file name of the `Body`.
    pub fn set_file_name<P>(&mut self, file_name: Option<P>)
    where
        P: AsRef<str>,
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
