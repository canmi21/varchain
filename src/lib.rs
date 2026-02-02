/* src/lib.rs */

//!
//! `varchain` is an async-only, zero-runtime-dependency crate for looking up variables
//! from a chain of sources.
//!
//! ## Core Concepts
//!
//! - **[Resolved]**: The result of a lookup from a single source (`Found` or `Pass`).
//! - **[Source]**: A trait for objects that can resolve a key to a `Resolved` value.
//! - **[Scope]**: A collection of sources queried in order.
//! - **[lookup]**: The primary function to query a key from a `Scope`.
//!
//! ## Example
//!
//! ```rust
//! use varchain::{Scope, Resolved, Source};
//! use std::collections::HashMap;
//!
//! # async fn run() {
//! let mut map = HashMap::new();
//! map.insert("key1".to_string(), "value1".to_string());
//!
//! let scope = Scope::new()
//!     .push(map); // HashMap impls Source
//!
//! assert_eq!(scope.lookup("key1").await, Some("value1".to_string()));
//! assert_eq!(scope.lookup("key2").await, None);
//! # }
//! ```

#![deny(missing_docs)]

mod resolved;
mod scope;
mod source;

pub use resolved::Resolved;
pub use scope::{Scope, lookup};
pub use source::{Source, SourceFuture};
