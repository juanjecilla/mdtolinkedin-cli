.PHONY: build release test check fmt fmt-check clippy ci bench audit deb rpm install

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

ci: fmt-check clippy test

bench:
	cargo bench

audit:
	cargo audit

deb:
	cargo deb

rpm:
	cargo rpm build

install:
	cargo install --path .
