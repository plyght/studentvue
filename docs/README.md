# Documentation

Welcome to the StudentVue Monorepo documentation.

## 📚 Table of Contents

### Getting Started

- [Main README](../README.md) - Project overview and quick start
- [Monorepo Guide](./MONOREPO_GUIDE.md) - Comprehensive guide to working with the monorepo

### Package Documentation

- [Rust API Library](../packages/studentvue-api/README.md) - Native Rust client library
- [MCP Server](../packages/studentvue-mcp/README.md) - AI integration via MCP protocol
- [MCP Installation Guide](../packages/studentvue-mcp/INSTALL.md) - Detailed installation instructions

### Contributing

- [Contributing Guide](./CONTRIBUTING.md) - Development workflow and standards

### Additional Resources

- [Setup Guide](../packages/studentvue-api/SETUP.md) - Rust API setup instructions
- [Changes Log](../packages/studentvue-api/CHANGES.md) - Version history

## Quick Links

### For Developers

**Working with Rust API:**
```bash
cd packages/studentvue-api
cargo build --release
```

**Working with MCP Server:**
```bash
cd packages/studentvue-mcp
bun install && bun run build
```

### For Contributors

1. Read the [Contributing Guide](./CONTRIBUTING.md)
2. Follow the [Monorepo Guide](./MONOREPO_GUIDE.md)
3. Run `make quality-gates` before submitting PRs

## External Documentation

- [Model Context Protocol](https://modelcontextprotocol.io/docs/getting-started/intro)
- [StudentVue API Docs](https://github.com/StudentVue/docs)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Bun Documentation](https://bun.sh/docs)

## Need Help?

- Check the [Monorepo Guide](./MONOREPO_GUIDE.md) for common workflows
- Review package-specific READMEs for API details
- Look at example code in `packages/studentvue-api/examples/`
- Check existing issues and discussions

