fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test mod="":
    cargo test {{mod}} --workspace --all-features

build:
    cargo build --workspace --all-targets

build-release:
    cargo build --workspace --release --all-targets

local-install:
    cargo install --path .