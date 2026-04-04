all: build test fmt clippy update-readme

build:
  cargo build

test:
  cargo test

fmt:
  cargo +nightly fmt

clippy:
  cargo clippy

update-readme:
  nix run nixpkgs#cargo-rdme
