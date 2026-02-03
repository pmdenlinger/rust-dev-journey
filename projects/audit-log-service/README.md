
# Audit Log Service

A small, reliable **append-only audit log** with a simple HTTP API.

## Why
Systems handling sensitive actions need a **tamper-evident** record of who did what and when.

## Highlights
- `axum` + `tokio` for async HTTP
- `serde` for JSON I/O
- `tracing` for structured logs (JSON in containers)
- Clear error types (`thiserror`), no `.unwrap()` in library code
- Graceful shutdown, health endpoints

## Run
```bash
cargo run -p audit-log-service
```

## Test
```bash
cargo test -p audit-log-service -- --nocapture
```

## Build container
```bash
docker build -t audit-log-service:local .
```

## Architecture Notes
- See `/docs/architecture/adr-0003-logging-and-tracing.md`
- Invariants: append-only, ordered entries, immutable records
