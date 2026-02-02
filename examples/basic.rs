/* examples/basic.rs */

use std::collections::HashMap;
use varchain::{Resolved, Scope};

#[tokio::main]
async fn main() {
	// 1. Define a HashMap source (L4)
	let mut l4_map = HashMap::new();
	l4_map.insert("host".to_owned(), "localhost".to_owned());
	l4_map.insert("port".to_owned(), "8080".to_owned());

	// 2. Define a Closure source (L7 / Hijacker)
	// This source intercepts "port" and returns "443" if the host is "secure.com"
	// For other keys, it passes.
	let hijacker = |key: &str| -> Resolved {
		if key == "hijacked" {
			return Resolved::Found("yes".to_owned());
		}
		Resolved::Pass
	};

	// 3. Build the Scope
	// Hijacker has higher priority (pushed first? No, wait. Scope::push adds to the end.)
	// Let's re-read the doc: "Adds Source to chain end (lowest priority). First pushed is higher priority?"
	// Checking Scope::push impl: `self.sources.push(...)`.
	// Checking lookup impl: `for source in &scope.sources`.
	// So: Index 0 is checked first.
	// `Scope::new().push(A).push(B)` -> sources: [A, B].
	// So A is checked first. A has higher priority.

	let scope = Scope::new()
		.push(hijacker) // Priority 0: Hijacker
		.push(l4_map); // Priority 1: Map

	// 4. Lookups

	// "hijacked" -> handled by hijacker
	let val = scope.lookup("hijacked").await;
	println!("hijacked: {val:?}"); // Some("yes")
	assert_eq!(val, Some("yes".to_owned()));

	// "host" -> hijacker passes, map handles
	let val = scope.lookup("host").await;
	println!("host: {val:?}"); // Some("localhost")
	assert_eq!(val, Some("localhost".to_owned()));

	// "missing" -> both pass
	let val = scope.lookup("missing").await;
	println!("missing: {val:?}"); // None
	assert_eq!(val, None);
}
