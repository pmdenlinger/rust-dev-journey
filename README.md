
# Rust Journey – Systems Reliability by Design

This repository documents my **Rust learning path** and the **systems** I build—focused on correctness, clarity, and cloud‑friendly delivery.

- **Examples** prove fundamentals (ownership, error handling, traits, async).
- **Projects** demonstrate production‑minded services, CLIs, and agents.
- **ADRs** record architecture decisions and trade‑offs.

---

## What to look at first
- **[SUMMARY.md](SUMMARY.md)** – quick index of examples and projects
- **[projects/](projects/)** – portfolio‑grade code with design notes
- **[docs/architecture](docs/architecture/)** – Architecture Decision Records (ADRs)

## Tech Focus
- Language: **Rust** (Edition 2021)
- Runtime & libs: **tokio**, **axum**, **serde**, **thiserror**, **tracing**
- Tooling: `cargo fmt`, `cargo clippy`, `cargo test`
- Cloud fit: **Azure** (containerized services, CI/CD), but vendor‑agnostic designs

## Status Badges
![CI](https://github.com/pmdenlinger/rust-dev-journey/actions/workflows/ci.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)

## Learning Philosophy
Progress from **small, focused examples** → **real projects**. Each step is documented with:
- Motivation
- Constraints and trade‑offs
- Tests and observability

---

## License
[MIT](LICENSE)
