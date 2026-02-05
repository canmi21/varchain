/* examples/ahash.rs */

//! Minimal example: using an `AHashMap` as a source.

use ahash::AHashMap;
use varchain::Scope;

#[tokio::main]
async fn main() {
	let mut map = AHashMap::new();
	map.insert("port".to_owned(), "8080".to_owned());

	let scope = Scope::new().push(map);

	let val = scope.lookup("port").await;
	println!("port: {val:?}");
	assert_eq!(val, Some("8080".to_owned()));
}
