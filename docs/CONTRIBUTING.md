# Contributing to StudentVue Monorepo

Thank you for your interest in contributing! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust** 1.70 or higher ([install](https://rustup.rs/))
- **Bun** latest version ([install](https://bun.sh/))
- **Git**

### Clone and Setup

```bash
git clone <repository-url>
cd studentvue
make install
```

## Project Structure

```
studentvue/
├── packages/
│   ├── studentvue-api/      # Rust library
│   └── studentvue-mcp/      # TypeScript MCP server
├── Cargo.toml               # Workspace configuration
└── Makefile                 # Build automation
```

## Development Workflow

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes in the appropriate package

3. Run quality gates:
   ```bash
   make quality-gates
   ```

4. Commit your changes:
   ```bash
   git add .
   git commit -m "Description of changes"
   ```

### Rust Package (packages/studentvue-api)

```bash
cd packages/studentvue-api

cargo build         # Build
cargo test          # Run tests
cargo fmt           # Format code
cargo clippy        # Lint code
cargo check         # Type check
```

### MCP Server (packages/studentvue-mcp)

```bash
cd packages/studentvue-mcp

bun install         # Install dependencies
bun run build       # Build
bun test            # Run tests
bun run format      # Format code
bun run lint        # Lint code
bun run typecheck   # Type check
```

## Code Standards

### Rust

- Use `rustfmt` for formatting (enforced)
- Pass all `clippy` lints with `-D warnings`
- Include doc comments for public APIs
- Write tests for new functionality
- Preserve all existing comments

### TypeScript

- Use Prettier for formatting (enforced)
- Pass ESLint checks
- Follow TypeScript strict mode
- Include JSDoc comments for public APIs
- Write tests for new functionality
- Preserve all existing comments

## Testing

### Writing Tests

- **Rust**: Add tests in `packages/studentvue-api/tests/`
- **TypeScript**: Add tests alongside source files with `.test.ts` extension

### Running Tests

```bash
make test                              # Run all tests
cd packages/studentvue-api && cargo test    # Rust tests only
cd packages/studentvue-mcp && bun test      # TypeScript tests only
```

## Quality Gates

All PRs must pass these checks:

```bash
make quality-gates
```

This runs:
1. Code formatting (rustfmt, prettier)
2. Linting (clippy, eslint)
3. Type checking (cargo check, tsc)
4. Tests (cargo test, bun test)

## Pull Request Process

1. Ensure all quality gates pass
2. Update documentation if needed
3. Add tests for new features
4. Update CHANGES.md with your changes
5. Create a pull request with a clear description

### PR Title Format

- `feat: Add new feature`
- `fix: Fix bug description`
- `docs: Update documentation`
- `chore: Maintenance tasks`
- `test: Add or update tests`

## Coding Guidelines

### General

- Never commit credentials or secrets
- No new license headers or banners
- No placeholder code in PRs
- Keep changes focused and atomic

### Rust Specific

- Use `?` operator for error propagation
- Prefer `async/await` over callbacks
- Use `Result<T>` for fallible operations
- Follow Rust naming conventions

### TypeScript Specific

- Use ES modules (`import`/`export`)
- Prefer `async/await` over promises
- Use proper type annotations
- Avoid `any` type when possible

## Documentation

- Update README.md if adding features
- Add inline comments for complex logic
- Keep existing comments intact
- Use clear, concise language

## Getting Help

- Check existing issues and PRs
- Review the README files in each package
- Look at existing code for examples

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

