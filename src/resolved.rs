//! Defines the resolution result of a variable lookup.

/// The result of resolving a key from a [`Source`](crate::Source).
///
/// This enum distinguishes between "key exists but value is empty" (`Found("")`)
/// and "key does not exist / not handled by this source" (`Pass`).
///
/// The type parameter `V` defaults to `String`, so `Resolved` and `Resolved<String>`
/// are equivalent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolved<V = String> {
	/// The source successfully resolved the key.
	///
	/// The value can be an empty string, which means the key exists but has no content.
	Found(V),

	/// The source does not handle this key or the key was not found.
	///
	/// The lookup should continue to the next source in the chain.
	Pass,
}

impl<V> Resolved<V> {
	/// Creates a `Resolved::Found` variant.
	#[must_use]
	pub fn found(v: impl Into<V>) -> Self {
		Self::Found(v.into())
	}

	/// Creates a `Resolved::Pass` variant.
	#[must_use]
	pub const fn pass() -> Self {
		Self::Pass
	}

	/// Returns `true` if this is a `Resolved::Found` variant.
	#[must_use]
	pub fn is_found(&self) -> bool {
		matches!(self, Self::Found(_))
	}

	/// Returns `true` if this is a `Resolved::Pass` variant.
	#[must_use]
	pub fn is_pass(&self) -> bool {
		matches!(self, Self::Pass)
	}

	/// Converts the `Resolved` into an `Option<V>`.
	///
	/// - `Found(v)` becomes `Some(v)`.
	/// - `Pass` becomes `None`.
	#[must_use]
	pub fn into_option(self) -> Option<V> {
		match self {
			Self::Found(v) => Some(v),
			Self::Pass => None,
		}
	}
}

/// Converts a value into `Resolved::Found`.
impl<V> From<V> for Resolved<V> {
	fn from(value: V) -> Self {
		Self::Found(value)
	}
}

/// Converts an `Option<V>` into `Resolved<V>`.
///
/// `Some(v)` becomes `Found(v)`, `None` becomes `Pass`.
impl<V> From<Option<V>> for Resolved<V> {
	fn from(opt: Option<V>) -> Self {
		match opt {
			Some(v) => Self::Found(v),
			None => Self::Pass,
		}
	}
}
