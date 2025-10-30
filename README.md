# StudentVue Monorepo

A comprehensive StudentVue API implementation providing both a Rust library and a Model Context Protocol (MCP) server. Access student information, grades, attendance, messages, and more through native Rust or AI-powered applications.

## 📦 Packages

### [StudentVue Rust API](./packages/studentvue-api/)
A type-safe Rust client library for the StudentVue SOAP API with 15+ fully implemented endpoints.

**Key Features:**
- Type-safe data models with proper error handling
- Async/await support via Tokio runtime
- Automatic SOAP envelope construction and XML parsing
- Production-ready with comprehensive test coverage

### [StudentVue MCP Server](./packages/studentvue-mcp/)
A Model Context Protocol server that enables AI applications like Claude to access StudentVue data.

**Key Features:**
- 15+ StudentVue API endpoints exposed as MCP tools
- Built with the official MCP TypeScript SDK
- Seamless integration with Claude Desktop and other MCP clients
- Secure credential handling via environment variables

## 🚀 Quick Start

### Using the Rust API

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

### Using the MCP Server

```bash
cd packages/studentvue-mcp
bun install
bun run build
```

Configure with Claude Desktop (`~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "studentvue": {
      "command": "node",
      "args": ["/path/to/studentvue/packages/studentvue-mcp/dist/index.js"],
      "env": {
        "STUDENTVUE_PORTAL": "https://your-district.edupoint.com",
        "STUDENTVUE_USERNAME": "your_username",
        "STUDENTVUE_PASSWORD": "your_password"
      }
    }
  }
}
```

Then ask Claude:
- "What are my current grades?"
- "Show me my attendance record"
- "What messages do I have from teachers?"

## 📋 Available API Methods

Both packages provide access to the same StudentVue features:

### Student Information
- Get student profile (name, grade, school, contact info)
- Get school information (principal, address, phone)

### Academic Records
- Get gradebook (grades, assignments, course info)
- Get class schedule (periods, teachers, rooms)
- Get calendar events
- Get homework notes (district-dependent)

### Attendance
- Get attendance records (absences, tardies, reasons)

### Communication
- Get messages from teachers/administrators
- Mark messages as read

### Documents
- List available documents
- Download specific documents
- List and download report cards

### Health Records
- Get student health information (conditions, visits, immunizations)

### Utilities
- Search for school districts by ZIP code

## 🛠️ Development

### Workspace Commands

From the repository root:

```bash
make build          # Build all packages
make test           # Run all tests
make format         # Format all code
make lint           # Run all linters
make typecheck      # Run type checking
make quality-gates  # Run all quality checks
make clean          # Clean build artifacts
```

### Per-Package Development

**Rust API:**
```bash
cd packages/studentvue-api
cargo build         # Build
cargo test          # Test
cargo fmt           # Format
cargo clippy        # Lint
```

**MCP Server:**
```bash
cd packages/studentvue-mcp
bun run build       # Build
bun test            # Test
bun run format      # Format
bun run lint        # Lint
bun run typecheck   # Type check
```

## 🏗️ Project Structure

```
studentvue/
├── packages/
│   ├── studentvue-api/      # Rust API library
│   │   ├── src/
│   │   │   ├── client.rs    # Main client implementation
│   │   │   ├── models.rs    # Data structures
│   │   │   ├── soap.rs      # SOAP protocol handling
│   │   │   └── error.rs     # Error types
│   │   ├── tests/           # Integration tests
│   │   └── examples/        # Usage examples
│   │
│   └── studentvue-mcp/      # MCP server
│       ├── src/
│       │   ├── index.ts              # MCP server implementation
│       │   └── studentvue-client.ts  # StudentVue API client
│       └── dist/            # Built artifacts
│
├── Cargo.toml               # Rust workspace config
├── Makefile                 # Build automation
└── README.md                # This file
```

## 🔒 Security Considerations

**Important:** Never commit credentials to version control. Both packages use environment variables for authentication:

- `STUDENTVUE_PORTAL` - Your district's StudentVue portal URL
- `STUDENTVUE_USERNAME` - Your student ID or username
- `STUDENTVUE_PASSWORD` - Your StudentVue password

Always use `.env` files (which are gitignored) or secure credential management systems.

## 🔍 Finding Your District Portal

Use the district lookup utility available in both packages:

**Rust:**
```rust
let districts = client.get_districts_by_zip("12345").await?;
```

**MCP Server:**
Ask Claude: "Find StudentVue districts for ZIP code 12345"

Your district portal URL typically follows the pattern `https://[district-name].edupoint.com`.

## 📖 Documentation

- [Monorepo Guide](./docs/MONOREPO_GUIDE.md) - Working with the monorepo
- [Contributing Guide](./docs/CONTRIBUTING.md) - How to contribute
- [Rust API Documentation](./packages/studentvue-api/README.md)
- [MCP Server Documentation](./packages/studentvue-mcp/README.md)
- [MCP Protocol Documentation](https://modelcontextprotocol.io/docs/getting-started/intro)
- [StudentVue API Documentation](https://github.com/StudentVue/docs)

## 🧪 Testing

Both packages include comprehensive test suites. Set up test credentials in `.env` files:

```bash
cd packages/studentvue-api && cargo test
cd packages/studentvue-mcp && bun test
```

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](./docs/CONTRIBUTING.md) for details.

All contributions must:
1. Be formatted properly (rustfmt for Rust, Prettier for TypeScript)
2. Pass all lints (clippy for Rust, ESLint for TypeScript)
3. Pass all tests
4. Include tests for new features

## 📄 License

MIT

## 🙏 Acknowledgments

Based on reverse-engineering and documentation of the StudentVue SOAP API by the community.

## 💡 Use Cases

### For Developers (Rust API)
- Build custom StudentVue integrations
- Create mobile apps with native performance
- Automate grade monitoring and notifications
- Integrate with school management systems

### For AI Applications (MCP Server)
- Enable Claude to answer questions about grades
- Get natural language summaries of attendance
- Ask about upcoming assignments and events
- Retrieve and explain report cards

## 🔗 Resources

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Claude Desktop](https://claude.ai/download)
- [Rust Documentation](https://www.rust-lang.org/)
- [Bun Runtime](https://bun.sh/)
