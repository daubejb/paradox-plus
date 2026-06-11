# ADR 0002: Graceful QUIC/TLS Configuration Stubbing

## Context
Authoritative server coordinator tests must run in headless environments (e.g. CI/CD or local test suites) to verify FSM transitions and action validations. Binding a live Quinn socket listener requires valid TLS certificates and private keys. During test execution, these certificates are absent. Generating self-signed certificates dynamically adds unnecessary dependency overhead (e.g., `rcgen`), and trying to bind a server socket without certificates causes immediate panics.

## Decision
We decided to implement a non-panicking, graceful configuration stub for the Quinn transport setup:
1. `configure_dummy_quic()` returns a clean `Result::Err` if TLS credentials are not configured or certificate parsing fails.
2. The server's background socket listener task catches the error and skips binding the live `Endpoint`, rather than panicking.
3. This allows the Bevy application runner to continue executing entirely via standard memory channels during tests and offline simulations.

## Consequences
* **Panics Avoided:** Headless test suites run without crashing due to network initialization failures.
* **Minimal Dependencies:** Avoids pulling in complex runtime self-signed certificate generation crates in testing profiles.
* **Separation of Concerns:** Keep network binding errors isolated from the gameplay state tick updates.
