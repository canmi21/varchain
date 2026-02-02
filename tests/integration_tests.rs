/* tests/integration_tests.rs */

use std::collections::{BTreeMap, HashMap};
use varchain::{Resolved, Scope, Source, SourceFuture};

#[tokio::test]
async fn test_basic_lookup_precedence() {
	let mut map1 = HashMap::new();
	map1.insert("key".to_owned(), "map1".to_owned());

	let mut map2 = HashMap::new();
	map2.insert("key".to_owned(), "map2".to_owned());
	map2.insert("only_in_2".to_owned(), "value2".to_owned());

	// scope order: map1, map2.
	// map1 should win for "key".
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
	let mut map = BTreeMap::new();
	map.insert("key".to_owned(), "value".to_owned());

	let scope = Scope::new().push(map);

	assert_eq!(scope.lookup("key").await, Some("value".to_owned()));
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

	// Push AsyncSource first, then map.
	// AsyncSource should win for "async".
	let scope = Scope::new().push(AsyncSource).push(map);

	assert_eq!(scope.lookup("async").await, Some("awaitable".to_owned()));
}

#[tokio::test]
async fn test_from_impls() {
	let s = "hello".to_owned();
	let r: Resolved = s.into();
	assert_eq!(r, Resolved::found("hello"));

	let r: Resolved = "world".into();
	assert_eq!(r, Resolved::found("world"));

	let opt = Some("opt".to_owned());
	let r: Resolved = opt.into();
	assert_eq!(r, Resolved::found("opt"));

	let opt: Option<String> = None;
	let r: Resolved = opt.into();
	assert_eq!(r, Resolved::pass());
}
