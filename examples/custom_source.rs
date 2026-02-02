//! Minimal example: implementing a custom async `Source`.

use varchain::{Resolved, Scope, Source, SourceFuture};

struct EnvSource;

impl Source for EnvSource {
	fn get(&self, key: &str) -> SourceFuture<'_> {
		// Simulating an asynchronous operation
		let key = key.to_owned();
		Box::pin(async move {
			if key == "mode" {
				Resolved::found("production")
			} else {
				Resolved::pass()
			}
		})
	}
}

#[tokio::main]
async fn main() {
	let scope = Scope::new().push(EnvSource);

	let val = scope.lookup("mode").await;
	println!("mode: {val:?}");
	assert_eq!(val, Some("production".to_owned()));
}
