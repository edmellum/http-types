//! Multipart/form-data types.
//!
//! # Examples
//!
//! Request:
//! ```
//! use http_types::multipart::{Multipart, Entry};
//!
//! let mut req = Request::new(Method::Get, "http://example.website");
//!
//! let mut multi = Multipart::new();
//! multi.push(Entry::new("description", "hello world"));
//!
//! let mut entry = Entry::from_file("my_file", Body::from_file("./cats.jpeg").await?);
//! entry.set_file_name("cats.jpeg");
//! multi.push("myFile", Body::from_file("./cats.jpeg").await?);
//!
//! req.set_body(multi);
//! ```
//!
//! Response:
//!
//! ```
//! use http_types::multipart::{Multipart, Entry};
//! let mut res = Response::new(200); // get this from somewhere
//!
//! let mut entries = res.body_multipart();
//! while let Some(entry) = entries.await {
//!     println!("name: {}", entry.name());
//!     println!("data: {}", entry.into_string()?);
//! }
//! ```

use crate::Body;
pub use entry::Entry;

mod entry;

/// A multipart response body.
#[derive(Debug)]
pub struct Multipart {
    entries: Vec<Entry>,
    body: Option<Body>,
}

impl Multipart {
    /// Create a new instance of `Multipart`.
    pub fn new() -> Self {
        Self {
            entries: vec![],
            body: None,
        }
    }

    /// Parse a `Body` stream as a `Multipart` instance.
    pub fn from_body(body: Body) -> Self {
        Self {
            entries: vec![],
            body: Some(body),
        }
    }

    /// Add a new entry to the `Multipart` instance.
    pub fn push<E>(&mut self, entry: E)
    where
        E: Into<Entry>,
    {
        self.entries.push(entry.into());
    }
}

// TODO
// impl Stream for Multipart {}

// TODO
// impl From<Multipart> for Body {}
