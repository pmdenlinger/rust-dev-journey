
# ADR-0002: Error Handling Policy
- **Status**: Accepted
- **Date**: 2026-02-03

## Context
Silent failures and ambiguous errors undermine reliability and auditability. We need a consistent approach for returning and observing errors across binaries and libraries.

## Decision
- Use `Result<T, E>` pervasively; avoid panics in library code.
- For binary crates, allow a single top-level `anyhow::Result<()>` to simplify main error bubbling.
- Define crate-local error enums with `thiserror` for typed errors and readable messages.
- Always attach context (`.with_context(...)` or custom variants) on fallible I/O and network calls.
- Convert errors to structured logs with fields: `error.kind`, `error.cause`, `correlation_id`.

## Alternatives Considered
- **Panic-first**: Fast to write, brittle in services and agents.
- **Only strings**: Easy, but loses programmatic handling and testability.

## Consequences
- **Positive**: Predictable control flow; testable failure paths; better observability.
- **Negative**: Slightly more code; discipline required.
- **Mitigation**: Provide helpers and examples; enforce via clippy and reviews.
