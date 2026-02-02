//! Minimal example: using a `HashMap` as a source.

use std::collections::HashMap;
use varchain::Scope;

#[tokio::main]
async fn main() {
	let mut map = HashMap::new();
	map.insert("port".to_owned(), "8080".to_owned());

	let scope = Scope::new().push(map);

	let val = scope.lookup("port").await;
	println!("port: {val:?}");
	assert_eq!(val, Some("8080".to_owned()));
}
