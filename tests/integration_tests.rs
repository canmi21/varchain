/* tests/integration_tests.rs */

//! Integration tests for varchain.
//!
//! ## Layer 1: Correctness + API stability
//! Verifies logic correctness and locks down the public API surface.
//!
//! ## Layer 2: Error paths
//! Verifies that edge cases return the correct result (None / Pass)
//! instead of panicking.

use std::collections::{BTreeMap, HashMap};
use varchain::{Resolved, Scope, Source, SourceFuture};

// ---------------------------------------------------------------------------
// Layer 1 – correctness & API stability
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_basic_lookup_precedence() {
	let mut map1 = HashMap::new();
	map1.insert("key".to_owned(), "map1".to_owned());

	let mut map2 = HashMap::new();
	map2.insert("key".to_owned(), "map2".to_owned());
	map2.insert("only_in_2".to_owned(), "value2".to_owned());

	let scope = Scope::new().push(map1).push(map2);

	assert_eq!(scope.lookup("key").await, Some("map1".to_owned()));
	assert_eq!(scope.lookup("only_in_2").await, Some("value2".to_owned()));
	assert_eq!(scope.lookup("missing").await, None);
}

#[tokio::test]
async fn test_empty_string_value() {
	let mut map = HashMap::new();
	map.insert("empty".to_owned(), "".to_owned());

	let scope = Scope::new().push(map);

	assert_eq!(scope.lookup("empty").await, Some("".to_owned()));
}

#[tokio::test]
async fn test_btreemap_source() {
	let mut btree = BTreeMap::new();
	btree.insert("key".to_owned(), "btree".to_owned());

	let mut map = HashMap::new();
	map.insert("key".to_owned(), "hash".to_owned());
	map.insert("only_hash".to_owned(), "h".to_owned());

	// btree first → wins for "key", fallback to map for "only_hash".
	let scope = Scope::new().push(btree).push(map);

	assert_eq!(scope.lookup("key").await, Some("btree".to_owned()));
	assert_eq!(scope.lookup("only_hash").await, Some("h".to_owned()));
}

#[tokio::test]
async fn test_closure_source() {
	let closure = |k: &str| {
		if k == "dynamic" {
			Resolved::found("computed")
		} else {
			Resolved::pass()
		}
	};

	let scope = Scope::new().push(closure);

	assert_eq!(scope.lookup("dynamic").await, Some("computed".to_owned()));
	assert_eq!(scope.lookup("other").await, None);
}

// Manual async source implementation
struct AsyncSource;
impl Source for AsyncSource {
	fn get(&self, key: &str) -> SourceFuture<'_> {
		let k = key.to_owned();
		Box::pin(async move {
			if k == "async" {
				Resolved::found("awaitable")
			} else {
				Resolved::pass()
			}
		})
	}
}

#[tokio::test]
async fn test_async_source_mixed() {
	let mut map = HashMap::new();
	map.insert("async".to_owned(), "shadowed_by_async_source".to_owned());

	let scope = Scope::new().push(AsyncSource).push(map);

	assert_eq!(scope.lookup("async").await, Some("awaitable".to_owned()));
}

#[tokio::test]
async fn test_from_impls() {
	// String → Found
	let s = "hello".to_owned();
	let r: Resolved = s.into();
	assert_eq!(r, Resolved::found("hello"));

	let r: Resolved = Resolved::found("world");
	assert_eq!(r, Resolved::Found("world".to_owned()));

	// Some → Found, None → Pass
	let opt = Some("opt".to_owned());
	let r: Resolved = opt.into();
	assert_eq!(r, Resolved::found("opt"));

	let opt: Option<String> = None;
	let r: Resolved = opt.into();
	assert_eq!(r, Resolved::pass());
}

#[tokio::test]
async fn test_resolved_into_option() {
	let found: Resolved = Resolved::found("value");
	assert_eq!(found.into_option(), Some("value".to_owned()));

	let pass: Resolved = Resolved::pass();
	assert_eq!(pass.into_option(), None);
}

#[tokio::test]
async fn test_resolved_is_found_is_pass() {
	let found: Resolved = Resolved::found("x");
	assert!(found.is_found());
	assert!(!found.is_pass());

	let pass: Resolved = Resolved::pass();
	assert!(pass.is_pass());
	assert!(!pass.is_found());
}

#[tokio::test]
async fn test_resolved_clone() {
	let original: Resolved = Resolved::found("cloned");
	let cloned = original.clone();
	assert_eq!(original, cloned);

	let original: Resolved = Resolved::pass();
	let cloned = original.clone();
	assert_eq!(original, cloned);
}

#[tokio::test]
async fn test_free_function_lookup() {
	let mut map = HashMap::new();
	map.insert("k".to_owned(), "v".to_owned());

	let scope = Scope::new().push(map);

	assert_eq!(varchain::lookup("k", &scope).await, Some("v".to_owned()));
	assert_eq!(varchain::lookup("missing", &scope).await, None);
}

#[tokio::test]
async fn test_scope_default() {
	let scope: Scope = Scope::default();
	assert_eq!(scope.lookup("anything").await, None);
}

#[tokio::test]
async fn test_scope_debug() {
	let scope: Scope = Scope::new();
	let dbg = format!("{:?}", scope);
	assert!(dbg.contains("Scope"));
	assert!(dbg.contains("0"));

	let mut map = HashMap::new();
	map.insert("k".to_owned(), "v".to_owned());
	let scope = scope.push(map);
	let dbg = format!("{:?}", scope);
	assert!(dbg.contains("1"));
}

#[tokio::test]
async fn test_scope_clone() {
	let mut map = HashMap::new();
	map.insert("key".to_owned(), "value".to_owned());

	let scope = Scope::new().push(map);
	let cloned = scope.clone();

	// Both scopes resolve the same key.
	assert_eq!(scope.lookup("key").await, Some("value".to_owned()));
	assert_eq!(cloned.lookup("key").await, Some("value".to_owned()));
}

#[tokio::test]
async fn test_generic_type_non_string() {
	let mut map: BTreeMap<String, i64> = BTreeMap::new();
	map.insert("answer".to_owned(), 42);

	let scope: Scope<i64> = Scope::new().push(map);

	assert_eq!(scope.lookup("answer").await, Some(42));
	assert_eq!(scope.lookup("missing").await, None);
}

#[cfg(feature = "ahash")]
#[tokio::test]
async fn test_ahash_source() {
	use ahash::AHashMap;

	let mut map1 = AHashMap::new();
	map1.insert("key1".to_owned(), "val1".to_owned());

	let mut map2 = HashMap::new();
	map2.insert("key1".to_owned(), "val2".to_owned());
	map2.insert("key2".to_owned(), "val2".to_owned());

	let scope = Scope::new().push(map1).push(map2);

	assert_eq!(scope.lookup("key1").await, Some("val1".to_owned()));
	assert_eq!(scope.lookup("key2").await, Some("val2".to_owned()));
}

// ---------------------------------------------------------------------------
// Layer 2 – error paths (return correct result, never panic)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_empty_scope_returns_none() {
	let scope: Scope = Scope::new();
	assert_eq!(scope.lookup("anything").await, None);
}

#[tokio::test]
async fn test_free_function_on_empty_scope() {
	let scope: Scope = Scope::new();
	assert_eq!(varchain::lookup("key", &scope).await, None);
}

#[tokio::test]
async fn test_all_sources_pass() {
	// Every source returns Pass for the queried key.
	let pass_closure_1 = |_: &str| Resolved::<String>::pass();
	let pass_closure_2 = |_: &str| Resolved::<String>::pass();

	let scope = Scope::new().push(pass_closure_1).push(pass_closure_2);
	assert_eq!(scope.lookup("key").await, None);
}

#[tokio::test]
async fn test_lookup_empty_key() {
	let mut map = HashMap::new();
	map.insert("".to_owned(), "empty_key_value".to_owned());

	let scope = Scope::new().push(map);

	// Empty string as key should work.
	assert_eq!(scope.lookup("").await, Some("empty_key_value".to_owned()));
}

#[tokio::test]
async fn test_lookup_empty_key_not_present() {
	let mut map = HashMap::new();
	map.insert("key".to_owned(), "value".to_owned());

	let scope = Scope::new().push(map);

	// Empty key not inserted → None, not panic.
	assert_eq!(scope.lookup("").await, None);
}
