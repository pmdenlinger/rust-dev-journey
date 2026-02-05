.PHONY: all ci fmt lint test build clean

all: fmt lint build

ci: fmt lint test

fmt:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -Dwarnings

test:
    cargo test --workspace --all-features

build:
    cargo build --workspace --all-features

clean:
    cargo clean