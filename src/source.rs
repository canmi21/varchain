/* src/source.rs */

use std::collections::{BTreeMap, HashMap};
use std::future::{Future, ready};
use std::pin::Pin;

use crate::Resolved;

/// A future returned by a source lookup.
pub type SourceFuture<'a> = Pin<Box<dyn Future<Output = Resolved> + Send + 'a>>;

/// A source capable of looking up a variable by key.
///
/// Implementors can range from simple in-memory maps to complex asynchronous
/// resolvers like network requests or file I/O.
pub trait Source: Send + Sync {
	/// Looks up a key.
	///
	/// Returns `Resolved::Found(value)` if the source provides a value for the key.
	/// Returns `Resolved::Pass` if the source does not handle the key.
	fn get(&self, key: &str) -> SourceFuture<'_>;
}

// Blanket implementation for HashMap
impl Source for HashMap<String, String> {
	/// Looks up a key in a `HashMap`. Returns `Found` if the key exists, `Pass` otherwise.
	fn get(&self, key: &str) -> SourceFuture<'_> {
		let res = match self.get(key) {
			Some(v) => Resolved::Found(v.clone()),
			None => Resolved::Pass,
		};
		Box::pin(ready(res))
	}
}

// Blanket implementation for BTreeMap
impl Source for BTreeMap<String, String> {
	/// Looks up a key in a `BTreeMap`. Returns `Found` if the key exists, `Pass` otherwise.
	fn get(&self, key: &str) -> SourceFuture<'_> {
		let res = match self.get(key) {
			Some(v) => Resolved::Found(v.clone()),
			None => Resolved::Pass,
		};
		Box::pin(ready(res))
	}
}

// Blanket implementation for Closures
impl<F> Source for F
where
	F: Fn(&str) -> Resolved + Send + Sync,
{
	/// Allows a closure that takes a `&str` and returns `Resolved` to be used as a `Source`.
	fn get(&self, key: &str) -> SourceFuture<'_> {
		Box::pin(ready(self(key)))
	}
}
