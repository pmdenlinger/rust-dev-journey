
# ADR-0003: Structured Logging & Tracing
- **Status**: Accepted
- **Date**: 2026-02-03

## Context
Text logs hinder searchability and correlation across distributed systems. We need structured, consistent logs with correlation IDs and spans for async work.

## Decision
- Use `tracing` + `tracing-subscriber` with `env-filter` and JSON format in containers.
- Standard fields: `service`, `version`, `correlation_id`, `request_id`, `span`, `error.kind`.
- Propagate correlation IDs via headers for HTTP (`x-correlation-id`) and via task-local storage for internal jobs.
- Include health and readiness endpoints in services; log lifecycle events (startup/shutdown) at `INFO` level.

## Alternatives Considered
- **println! debugging**: Simple but unstructured and hard to analyze.
- **Third-party agents only**: Useful, but local structure still required for portability.

## Consequences
- **Positive**: Faster diagnosis; production-ready defaults; portable across clouds.
- **Negative**: Slight learning curve; additional dependencies.
- **Mitigation**: Provide starter code in projects; document log fields in `docs/style-guide.md`.
