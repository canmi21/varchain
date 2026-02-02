/* src/scope.rs */

use std::sync::Arc;

use crate::{Resolved, Source};

/// A chain of sources to be queried in order.
///
/// Sources are queried from first to last (insertion order).
/// The first source that returns `Resolved::Found` determines the result.
#[derive(Clone)]
pub struct Scope {
	sources: Vec<Arc<dyn Source>>,
}

impl Scope {
	/// Creates a new, empty `Scope`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			sources: Vec::new(),
		}
	}

	/// Adds a source to the end of the chain.
	///
	/// This source will be queried only if all previous sources return `Resolved::Pass`.
	#[must_use]
	pub fn push(mut self, source: impl Source + 'static) -> Self {
		self.sources.push(Arc::new(source));
		self
	}

	/// Looks up a key in this scope.
	///
	/// Equivalent to calling [`lookup`](crate::lookup) with this scope.
	pub async fn lookup(&self, key: &str) -> Option<String> {
		crate::lookup(key, self).await
	}
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

/// Looks up a key in the given scope.
///
/// Iterates through the sources in the scope. Returns `Some(value)` immediately
/// upon encountering `Resolved::Found(value)`. If a source returns `Resolved::Pass`,
/// the next source is queried. If all sources pass, returns `None`.
pub async fn lookup(key: &str, scope: &Scope) -> Option<String> {
	for source in &scope.sources {
		match source.get(key).await {
			Resolved::Found(val) => return Some(val),
			Resolved::Pass => {}
		}
	}
	None
}
impl std::fmt::Debug for Scope {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Scope")
			.field("sources_count", &self.sources.len())
			.finish()
	}
}
