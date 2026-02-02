# varchain

Async-only, zero-runtime-dependency variable chain lookup engine.

`varchain` provides a flexible way to resolve variables from a prioritized chain of sources (e.g., in-memory maps, environment variables, network lookups) without enforcing a specific runtime.

## Features

- **Async-Only**: Designed for asynchronous contexts from the ground up, compatible with any runtime (Tokio, async-std, smol, etc.) or just `core::future`.
- **Zero Runtime Dependencies**: Depends only on `std` and `thiserror`/`tracing`. No heavy runtime baggage.
- **Flexible Sources**: Easy implementation of custom sources via the `Source` trait.
- **Zero-Cost Abstractions**: Blanket implementations for `HashMap`, `BTreeMap`, and closures use `core::future::ready` for immediate resolution.
- **Composability**: Build lookup scopes by chaining multiple sources with strict precedence.

## Usage Examples

Check the `examples` directory for runnable code:

- **Basic Usage**: [`examples/basic.rs`](examples/basic.rs) - Demonstrate lookup precedence with HashMap and Closures.

## Installation

```toml
[dependencies]
varchain = { version = "0.0.1" }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `full` | Enables all features above. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)