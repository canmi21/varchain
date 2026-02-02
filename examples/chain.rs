/* examples/chain.rs */

use std::collections::HashMap;
use varchain::Scope;

#[tokio::main]
async fn main() {
	let mut high_priority = HashMap::new();
	high_priority.insert("theme".to_owned(), "dark".to_owned());

	let mut low_priority = HashMap::new();
	low_priority.insert("theme".to_owned(), "light".to_owned());
	low_priority.insert("font".to_owned(), "monospace".to_owned());

	// Earlier pushes have higher priority
	let scope = Scope::new().push(high_priority).push(low_priority);

	// "theme" is found in high_priority, so low_priority is ignored
	let theme = scope.lookup("theme").await;
	println!("theme: {theme:?}");
	assert_eq!(theme, Some("dark".to_owned()));

	// "font" is not in high_priority, so it falls back to low_priority
	let font = scope.lookup("font").await;
	println!("font: {font:?}");
	assert_eq!(font, Some("monospace".to_owned()));
}
