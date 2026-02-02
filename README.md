# Varchain

Async-only chain-based variable lookup engine, support no_std with alloc.

`varchain` provides a flexible way to resolve variables from a prioritized chain of sources (e.g., in-memory maps, environment variables, network lookups) without enforcing a specific runtime.

## Features

- **Async-Only**: Designed for asynchronous contexts from the ground up, compatible with any runtime (Tokio, async-std, smol, etc.) or just `core::future`.
- **Flexible Sources**: Easy implementation of custom sources via the `Source` trait.
- **Composability**: Build lookup scopes by chaining multiple sources with strict precedence.

## Usage Examples

Check the `examples` directory for runnable code:

- **HashMap Source**: [`examples/hashmap.rs`](examples/hashmap.rs) - Simple memory-based lookup.
- **Closure Source**: [`examples/closure.rs`](examples/closure.rs) - Use functions for dynamic lookups.
- **Chained Priority**: [`examples/chain.rs`](examples/chain.rs) - Demonstrate fallback and precedence.
- **Custom Source**: [`examples/custom_source.rs`](examples/custom_source.rs) - Implement the `Source` trait for async backends.

## Installation

```toml
[dependencies]
varchain = { version = "0.1", features = ["full"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` | Enables standard library support for hashmap. |
| `full` | Enables all features above. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)
