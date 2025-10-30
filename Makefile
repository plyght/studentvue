.PHONY: build test format lint typecheck quality-gates clean

build:
	cargo build --release

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

typecheck:
	cargo check

quality-gates: format lint typecheck test

clean:
	cargo clean

