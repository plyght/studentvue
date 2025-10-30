.PHONY: build test format lint typecheck quality-gates clean install

build:
	@echo "Building Rust API..."
	cd packages/studentvue-api && cargo build --release
	@echo "Building MCP Server..."
	cd packages/studentvue-mcp && bun run build

test:
	@echo "Testing Rust API..."
	cd packages/studentvue-api && cargo test
	@echo "Testing MCP Server..."
	cd packages/studentvue-mcp && bun test

format:
	@echo "Formatting Rust code..."
	cd packages/studentvue-api && cargo fmt
	@echo "Formatting TypeScript code..."
	cd packages/studentvue-mcp && bun run format

lint:
	@echo "Linting Rust code..."
	cd packages/studentvue-api && cargo clippy -- -D warnings
	@echo "Linting TypeScript code..."
	cd packages/studentvue-mcp && bun run lint

typecheck:
	@echo "Type checking Rust..."
	cd packages/studentvue-api && cargo check
	@echo "Type checking TypeScript..."
	cd packages/studentvue-mcp && bun run typecheck

quality-gates: format lint typecheck test
	@echo "All quality gates passed!"

install:
	@echo "Installing Rust dependencies..."
	cd packages/studentvue-api && cargo fetch
	@echo "Installing Node dependencies..."
	cd packages/studentvue-mcp && bun install

clean:
	@echo "Cleaning Rust build artifacts..."
	cd packages/studentvue-api && cargo clean
	@echo "Cleaning Node build artifacts..."
	cd packages/studentvue-mcp && rm -rf dist node_modules
	@echo "Clean complete!"
