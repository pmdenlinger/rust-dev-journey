
# ADR-0001: Use a Cargo Workspace Monorepo
- **Status**: Accepted
- **Date**: 2026-02-03

## Context
This repository must present a clear learning → portfolio progression while keeping builds, tests, and linting fast and consistent. Managing many independent repos increases friction and fragments CI.

## Decision
Adopt a **single Cargo workspace** at the repo root with members for examples and projects. Run `cargo fmt`, `cargo clippy`, and `cargo test` **once** across the workspace. Use a single CI workflow for uniform quality gates.

## Alternatives Considered
- **Multiple repos**: Clear separation but heavy on boilerplate, duplicated CI, and harder cross-crate reuse.
- **Single repo without workspace**: Simpler initial setup but loses shared dependency resolution and single-command checks.

## Consequences
- **Positive**: One build graph; consistent tooling; easier refactors; shared conventions.
- **Negative**: Larger checkout size over time; must keep examples and projects tidy.
- **Mitigation**: Archive stale sandboxes; keep `SUMMARY.md` and `ROADMAP.md` current.
