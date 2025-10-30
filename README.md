# StudentVue Monorepo

A comprehensive StudentVue API implementation providing both a Rust library and a Model Context Protocol (MCP) server. Access student information, grades, attendance, messages, and more through native Rust or AI-powered applications.

## Packages

### StudentVue Rust API

A type-safe Rust client library for the StudentVue SOAP API.

**Location:** [`packages/studentvue-api/`](./packages/studentvue-api/)

**Features:**
- Type-safe data models with proper error handling
- Async/await support via Tokio runtime
- Automatic SOAP envelope construction and XML parsing
- 15+ fully implemented API endpoints
- Production-ready with comprehensive test coverage

### StudentVue MCP Server

A Model Context Protocol server enabling AI applications like Claude to access StudentVue data.

**Location:** [`packages/studentvue-mcp/`](./packages/studentvue-mcp/)

**Features:**
- 15+ StudentVue API endpoints exposed as MCP tools
- Built with the official MCP TypeScript SDK
- Seamless integration with Claude Desktop and other MCP clients
- Secure credential handling via environment variables

## Quick Start

### Rust API

```bash
cd packages/studentvue-api
cargo build --release
```

Add to your `Cargo.toml`:

```toml
[dependencies]
studenvue = "0.1.0"
tokio = { version = "1.35", features = ["full"] }
```

Example usage:

```rust
use studenvue::{StudentVueClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = StudentVueClient::new(portal, username, password);
    let info = client.get_student_info().await?;
    println!("Student: {} (Grade {})", info.name, info.grade);
    Ok(())
}
```

### MCP Server

#### Quick Install for Cursor

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en-US/install-mcp?name=studentvue&config=eyJjb21tYW5kIjoibm9kZSIsImFyZ3MiOlsiL3BhdGgvdG8vc3R1ZGVudHZ1ZS9wYWNrYWdlcy9zdHVkZW50dnVlLW1jcC9kaXN0L2luZGV4LmpzIl0sImVudiI6eyJTVFVERU5UVlVFX1BPUlRBTCI6Imh0dHBzOi8veW91ci1kaXN0cmljdC5lZHVwb2ludC5jb20iLCJTVFVERU5UVlVFX1VTRVJOQU1FIjoieW91cl91c2VybmFtZSIsIlNUVURFTlRWVUVfUEFTU1dPUkQiOiJ5b3VyX3Bhc3N3b3JkIn19)

#### Manual Installation

```bash
cd packages/studentvue-mcp
bun install
bun run build
```

See [packages/studentvue-mcp/INSTALL.md](./packages/studentvue-mcp/INSTALL.md) for complete installation instructions for Claude Desktop, Cursor, and other MCP clients.

## Available API Methods

Both packages provide access to the following StudentVue features:

**Student Information**
- Get student profile (name, grade, school, contact info)
- Get school information (principal, address, phone)

**Academic Records**
- Get gradebook (grades, assignments, course info)
- Get class schedule (periods, teachers, rooms)
- Get calendar events
- Get homework notes (district-dependent)

**Attendance**
- Get attendance records (absences, tardies, reasons)

**Communication**
- Get messages from teachers/administrators
- Mark messages as read

**Documents**
- List available documents
- Download specific documents
- List and download report cards

**Health Records**
- Get student health information (conditions, visits, immunizations)

**Utilities**
- Search for school districts by ZIP code

## Development

### Workspace Commands

```bash
make build          # Build all packages
make test           # Run all tests
make format         # Format all code
make lint           # Run all linters
make typecheck      # Run type checking
make quality-gates  # Run all quality checks
make clean          # Clean build artifacts
make install        # Install dependencies
```

### Per-Package Development

**Rust API:**
```bash
cd packages/studentvue-api
cargo build && cargo test
```

**MCP Server:**
```bash
cd packages/studentvue-mcp
bun install && bun run build && bun test
```

## Project Structure

```
studentvue/
├── packages/
│   ├── studentvue-api/      # Rust API library
│   │   ├── src/             # Source code
│   │   ├── tests/           # Integration tests
│   │   └── examples/        # Usage examples
│   └── studentvue-mcp/      # MCP server
│       ├── src/             # TypeScript source
│       └── dist/            # Built artifacts
├── docs/                    # Documentation
├── Cargo.toml               # Rust workspace config
├── Makefile                 # Build automation
└── README.md                # This file
```

## Configuration

Both packages require StudentVue credentials via environment variables:

```bash
STUDENTVUE_PORTAL=https://your-district.edupoint.com
STUDENTVUE_USERNAME=your_username
STUDENTVUE_PASSWORD=your_password
```

**Security:** Never commit credentials to version control. Always use environment variables or `.env` files (gitignored by default).

## Documentation

- [Monorepo Guide](./docs/MONOREPO_GUIDE.md) - Working with the monorepo
- [Contributing Guide](./docs/CONTRIBUTING.md) - Development workflow and standards
- [Rust API Documentation](./packages/studentvue-api/README.md)
- [MCP Server Documentation](./packages/studentvue-mcp/README.md)
- [MCP Protocol Documentation](https://modelcontextprotocol.io/docs/getting-started/intro)

## Testing

Both packages include comprehensive test suites:

```bash
cd packages/studentvue-api && cargo test
cd packages/studentvue-mcp && bun test
```

Set up test credentials in `.env` files before running tests.

## Contributing

Contributions are welcome. Please see our [Contributing Guide](./docs/CONTRIBUTING.md) for details.

All contributions must:
- Be formatted properly (rustfmt for Rust, Prettier for TypeScript)
- Pass all lints (clippy for Rust, ESLint for TypeScript)
- Pass all tests
- Include tests for new features

## Use Cases

**For Developers (Rust API)**
- Build custom StudentVue integrations
- Create mobile apps with native performance
- Automate grade monitoring and notifications
- Integrate with school management systems

**For AI Applications (MCP Server)**
- Enable Claude to answer questions about grades
- Get natural language summaries of attendance
- Ask about upcoming assignments and events
- Retrieve and explain report cards

## License

MIT

## Acknowledgments

Based on reverse-engineering and documentation of the StudentVue SOAP API by the community.
