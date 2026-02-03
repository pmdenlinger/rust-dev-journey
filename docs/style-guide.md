
# Code Style & Conventions

- Run `cargo fmt`, `cargo clippy`, and `cargo test` before commits.
- Prefer explicit types at boundaries; avoid turbofish where it harms readability.
- No `unwrap()`/`expect()` in library code; prefer typed errors.
- Use `tracing` spans for async request handlers and long-running tasks.
- Keep modules small and named by responsibility: `routes`, `models`, `storage`, `error`.
