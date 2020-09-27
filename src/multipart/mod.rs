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

use std::task::Context;
use std::task::Poll;
use std::{fmt::Debug, pin::Pin, str::FromStr};

use futures_core::stream::Stream;
use multipart::server::Multipart as Parser;
use std::io::{Cursor, Read};

use crate::{format_err, Mime, Status};
pub use entry::Entry;

mod entry;

/// A multipart response body.
pub struct Multipart {
    entries: Vec<Entry>,
    body: Option<Parser<Cursor<String>>>,
}

impl Debug for Multipart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Multipart").finish()
    }
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
    pub async fn from_req(req: &mut crate::Request) -> crate::Result<Self> {
        let body = req.take_body().into_string().await?;
        let boundary = req
            .content_type()
            .map(|ct| ct.param("boundary").cloned())
            .flatten();

        let boundary = match boundary {
            Some(boundary) => boundary.as_str().to_owned(),
            None => {
                let mut err =
                    format_err!("Invalid `Content-Type` header. Expected a `boundary` param");
                err.set_status(400);
                return Err(err);
            }
        };

        let multipart = Parser::with_body(Cursor::new(body), boundary);
        Ok(Self {
            entries: vec![],
            body: Some(multipart),
        })
    }

    /// Add a new entry to the `Multipart` instance.
    pub fn push<E>(&mut self, entry: E)
    where
        E: Into<Entry>,
    {
        self.entries.push(entry.into());
    }
}

impl Stream for Multipart {
    type Item = crate::Result<Entry>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
        let body = match self.body.as_mut() {
            None => return Poll::Ready(None),
            Some(body) => body,
        };

        match body.read_entry() {
            Ok(Some(mut field)) => {
                let mut body = vec![];
                field.data.read_to_end(&mut body).status(400)?;

                let mut entry = Entry::new(field.headers.name, body);
                entry.set_file_name(field.headers.filename);
                let mime = field
                    .headers
                    .content_type
                    .map(|ct| Mime::from_str(&ct.to_string()))
                    .transpose()?;
                entry.set_content_type(mime);

                Poll::Ready(Some(Ok(entry)))
            }
            Ok(None) => Poll::Ready(None),
            Err(_e) => {
                // TODO: forward error?
                let mut err = format_err!("Invalid multipart entry");
                err.set_status(400);
                Poll::Ready(Some(Err(err)))
            }
        }
    }
}

// TODO
// impl From<Multipart> for Body {}
