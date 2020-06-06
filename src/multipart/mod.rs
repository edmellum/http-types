//! Multipart/form-data types.
//!
//! # Examples
//!
//! Request:
//! ```
//! let mut req = Request::new(Method::Get, "http://example.website");
//!
//! let mut multi = Multipart::new();
//! multi.push("hello world");
//! multi.push(Body::from_file("./cats.jpeg").await?);
//!
//! req.set_body(multi);
//! ```
//!
//! Response:
//!
//! ```
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
    pub fn push(&mut self, name: impl AsRef<str>, body: impl Into<Body>) {
        let entry = Entry::new(name, body);
        self.entries.push(entry);
    }
}

// TODO
// impl Stream for Multipart {}

// TODO
// impl From<Multipart> for Body {}
