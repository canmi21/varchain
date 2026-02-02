/* examples/closure.rs */

//! Minimal example: using a closure as a source.

use varchain::{Resolved, Scope};

#[tokio::main]
async fn main() {
	let closure = |key: &str| {
		if key == "version" {
			Resolved::found("1.0.0")
		} else {
			Resolved::pass()
		}
	};

	let scope = Scope::new().push(closure);

	let val = scope.lookup("version").await;
	println!("version: {val:?}");
	assert_eq!(val, Some("1.0.0".to_owned()));
}
