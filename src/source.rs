/* src/source.rs */

//! Defines the `Source` trait for variable lookup.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::future::{Future, ready};
use core::pin::Pin;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "ahash")]
use ahash::AHashMap;

use crate::Resolved;

/// A future returned by a source lookup.
pub type SourceFuture<'a, V = String> = Pin<Box<dyn Future<Output = Resolved<V>> + Send + 'a>>;

/// A source capable of looking up a variable by key.
///
/// Implementors can range from simple in-memory maps to complex asynchronous
/// resolvers like network requests or file I/O.
///
/// The type parameter `V` defaults to `String`.
pub trait Source<V = String>: Send + Sync {
	/// Looks up a key.
	///
	/// Returns `Resolved::Found(value)` if the source provides a value for the key.
	/// Returns `Resolved::Pass` if the source does not handle the key.
	fn get(&self, key: &str) -> SourceFuture<'_, V>;
}

/// Looks up a key in a `HashMap`. Returns `Found` if the key exists, `Pass` otherwise.
#[cfg(feature = "std")]
impl<V: Clone + Send + Sync> Source<V> for HashMap<String, V> {
	fn get(&self, key: &str) -> SourceFuture<'_, V> {
		let res = match self.get(key) {
			Some(v) => Resolved::Found(v.clone()),
			None => Resolved::Pass,
		};
		Box::pin(ready(res))
	}
}

/// Looks up a key in an `AHashMap`. Returns `Found` if the key exists, `Pass` otherwise.
#[cfg(feature = "ahash")]
impl<V: Clone + Send + Sync> Source<V> for AHashMap<String, V> {
	fn get(&self, key: &str) -> SourceFuture<'_, V> {
		let res = match Self::get(self, key) {
			Some(v) => Resolved::Found(v.clone()),
			None => Resolved::Pass,
		};
		Box::pin(ready(res))
	}
}

/// Looks up a key in a `BTreeMap`. Returns `Found` if the key exists, `Pass` otherwise.
impl<V: Clone + Send + Sync> Source<V> for BTreeMap<String, V> {
	fn get(&self, key: &str) -> SourceFuture<'_, V> {
		let res = match self.get(key) {
			Some(v) => Resolved::Found(v.clone()),
			None => Resolved::Pass,
		};
		Box::pin(ready(res))
	}
}

/// Allows a closure that takes a `&str` and returns `Resolved<V>` to be used as a `Source`.
impl<F, V> Source<V> for F
where
	F: Fn(&str) -> Resolved<V> + Send + Sync,
	V: Send + 'static,
{
	fn get(&self, key: &str) -> SourceFuture<'_, V> {
		Box::pin(ready(self(key)))
	}
}
