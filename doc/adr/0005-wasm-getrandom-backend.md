# ADR 0005: WASM build target getrandom backend configuration

## Context
The client package is designed to target both native desktop targets and WebAssembly (`wasm32-unknown-unknown`) browsers.
Starting with version 0.3.0, the `getrandom` crate (which Bevy depends on internally for UUIDs, random entities, and identifiers) does not support WebAssembly targets out-of-the-box. Compiling for WebAssembly without explicit configuration results in a compilation panic in getrandom's build backend:
`error: The wasm32-unknown-unknown targets are not supported by default; you may need to enable the "wasm_js" configuration flag.`

## Decision
To enable clean compilation for the client package on `wasm32-unknown-unknown` targets, we made two configurations:
1. Added a direct dependency on `getrandom` in the client's `Cargo.toml` with the `"wasm_js"` feature enabled:
   ```toml
   getrandom = { version = "0.3", features = ["wasm_js"] }
   ```
   This propagates the feature selection through Cargo's dependency graph.
2. Documented and mandated that WebAssembly builds and checks compile with the `getrandom_backend` configuration flag enabled via `RUSTFLAGS`:
   ```bash
   RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build -p client --target wasm32-unknown-unknown
   ```

## Consequences
* **WASM Target Support:** The client and protocol packages now compile cleanly and successfully for WebAssembly targets.
* **Feature Consistency:** No standard library features are compromised, and the authoritative server remains fully compatible without modifying getrandom dependencies.
