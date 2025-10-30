# StudentVue Monorepo Guide

This guide explains the monorepo structure and how to work with both packages.

## What's Inside

This monorepo contains two packages that provide different ways to access the StudentVue API:

1. **Rust API Library** (`packages/studentvue-api`) - For native Rust applications
2. **MCP Server** (`packages/studentvue-mcp`) - For AI applications like Claude

## Quick Start

### First Time Setup

```bash
git clone <repository-url>
cd studentvue
make install
```

This will:
- Fetch Rust dependencies
- Install Node.js dependencies via Bun

### Building Everything

```bash
make build
```

This builds both the Rust library and the MCP server.

## Working with Individual Packages

### Rust API Library

Located in `packages/studentvue-api/`

**Use this when:**
- Building native Rust applications
- Need maximum performance
- Want type-safe API bindings
- Developing CLI tools or services

**Quick Start:**
```bash
cd packages/studentvue-api
cp env.example .env
# Edit .env with your credentials
cargo build --release
cargo run --example basic_usage
```

**Documentation:** See [packages/studentvue-api/README.md](packages/studentvue-api/README.md)

### MCP Server

Located in `packages/studentvue-mcp/`

**Use this when:**
- Integrating with AI applications (Claude, ChatGPT, etc.)
- Building conversational interfaces
- Want natural language access to StudentVue data
- Need to expose StudentVue to MCP-compatible tools

**Quick Start:**
```bash
cd packages/studentvue-mcp
bun install
bun run build
# Configure with Claude Desktop (see README)
```

**Documentation:** See [packages/studentvue-mcp/README.md](packages/studentvue-mcp/README.md)

## Development Commands

### Workspace-Level Commands (from root)

```bash
make build          # Build all packages
make test           # Test all packages
make format         # Format all code
make lint           # Lint all code
make typecheck      # Type check all code
make quality-gates  # Run all checks
make clean          # Clean build artifacts
make install        # Install dependencies
```

### Package-Specific Commands

**Rust API:**
```bash
cd packages/studentvue-api
cargo build         # Build
cargo test          # Test
cargo fmt           # Format
cargo clippy        # Lint
cargo check         # Type check
cargo doc --open    # Generate and open docs
```

**MCP Server:**
```bash
cd packages/studentvue-mcp
bun install         # Install dependencies
bun run build       # Build
bun test            # Test
bun run format      # Format
bun run lint        # Lint
bun run typecheck   # Type check
bun run dev         # Run in development mode
```

## Common Workflows

### Adding a New API Endpoint

1. Add to Rust library:
   ```bash
   cd packages/studentvue-api
   # Edit src/client.rs to add new method
   # Edit src/models.rs to add data types
   cargo test
   ```

2. Add to MCP server:
   ```bash
   cd packages/studentvue-mcp
   # Edit src/studentvue-client.ts to add method
   # Edit src/index.ts to add MCP tool
   bun run typecheck
   bun run build
   ```

3. Test both implementations
4. Update documentation

### Testing Changes

```bash
make quality-gates
```

This ensures:
- ✅ Code is properly formatted
- ✅ No linting errors
- ✅ Type checking passes
- ✅ All tests pass

### Creating a Release

1. Update version in both packages:
   - `packages/studentvue-api/Cargo.toml`
   - `packages/studentvue-mcp/package.json`

2. Update CHANGES.md

3. Run quality gates:
   ```bash
   make quality-gates
   ```

4. Build release artifacts:
   ```bash
   make build
   ```

5. Commit and tag:
   ```bash
   git add .
   git commit -m "Release v0.x.0"
   git tag v0.x.0
   git push origin main --tags
   ```

## Credentials Management

Both packages require StudentVue credentials:

```bash
STUDENTVUE_PORTAL=https://your-district.edupoint.com
STUDENTVUE_USERNAME=your_username
STUDENTVUE_PASSWORD=your_password
```

**Set up for Rust API:**
```bash
cd packages/studentvue-api
cp env.example .env
# Edit .env with your credentials
```

**Set up for MCP Server:**
Configure in Claude Desktop config or set environment variables when running.

**⚠️ Security:** Never commit `.env` files or credentials to git!

## Troubleshooting

### Rust Build Issues

```bash
cd packages/studentvue-api
cargo clean
cargo update
cargo build
```

### TypeScript Build Issues

```bash
cd packages/studentvue-mcp
rm -rf node_modules dist
bun install
bun run build
```

### Make Command Not Found

Ensure you're in the repository root:
```bash
cd /path/to/studentvue
make build
```

### MCP Server Not Working

1. Verify build succeeded:
   ```bash
   cd packages/studentvue-mcp
   bun run build
   ls -la dist/index.js
   ```

2. Check Claude Desktop config is correct
3. Verify credentials are set
4. Check Claude Desktop logs

## Architecture

### Package Dependencies

```
studentvue/ (workspace root)
├── packages/studentvue-api/
│   └── Independent Rust library
│       No dependencies on other packages
│
└── packages/studentvue-mcp/
    └── Independent TypeScript MCP server
        No dependencies on other packages
```

Both packages are independent and can be used separately.

### Shared Concepts

Both packages implement the same StudentVue SOAP API:
- Same endpoints
- Same authentication
- Same data structures (equivalent)
- Different implementation languages

## Best Practices

1. **Always run quality gates before committing:**
   ```bash
   make quality-gates
   ```

2. **Keep packages independent:**
   - Don't add cross-package dependencies
   - Each package should work standalone

3. **Update documentation:**
   - Update package READMEs when adding features
   - Keep CHANGES.md current

4. **Test thoroughly:**
   - Add tests for new features
   - Run tests in both packages

5. **Follow language conventions:**
   - Rust: Follow Rust naming conventions
   - TypeScript: Follow TypeScript/JavaScript conventions

## Resources

- [Main README](../README.md)
- [Contributing Guide](./CONTRIBUTING.md)
- [Rust API README](../packages/studentvue-api/README.md)
- [MCP Server README](../packages/studentvue-mcp/README.md)
- [MCP Protocol Docs](https://modelcontextprotocol.io/)

## Getting Help

- Check package-specific READMEs
- Review example code
- Check existing issues
- Read the StudentVue API documentation

