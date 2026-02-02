/* src/resolved.rs */

/// The result of resolving a key from a [`Source`](crate::Source).
///
/// This enum distinguishes between "key exists but value is empty" (`Found("")`)
/// and "key does not exist / not handled by this source" (`Pass`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolved {
	/// The source successfully resolved the key.
	///
	/// The value can be an empty string, which means the key exists but has no content.
	Found(String),

	/// The source does not handle this key or the key was not found.
	///
	/// The lookup should continue to the next source in the chain.
	Pass,
}

impl Resolved {
	/// Creates a `Resolved::Found` variant.
	#[must_use]
	pub fn found(v: impl Into<String>) -> Self {
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

	/// Converts the `Resolved` into an `Option<String>`.
	///
	/// - `Found(v)` becomes `Some(v)`.
	/// - `Pass` becomes `None`.
	#[must_use]
	pub fn into_option(self) -> Option<String> {
		match self {
			Self::Found(v) => Some(v),
			Self::Pass => None,
		}
	}
}

impl From<String> for Resolved {
	/// Converts a `String` into a `Resolved::Found`.
	fn from(value: String) -> Self {
		Self::Found(value)
	}
}

impl From<&str> for Resolved {
	/// Converts a `&str` into a `Resolved::Found` by owning the string.
	fn from(value: &str) -> Self {
		Self::Found(value.to_owned())
	}
}

impl From<Option<String>> for Resolved {
	/// Converts an `Option<String>` into `Resolved`.
	/// `Some(v)` becomes `Found(v)`, `None` becomes `Pass`.
	fn from(opt: Option<String>) -> Self {
		match opt {
			Some(v) => Self::Found(v),
			None => Self::Pass,
		}
	}
}
