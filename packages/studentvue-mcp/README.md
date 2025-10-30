# StudentVue MCP Server

A Model Context Protocol (MCP) server that provides AI applications with access to the StudentVue API. This enables AI assistants like Claude to retrieve student information, grades, attendance, messages, and more.

## Features

- 15+ StudentVue API endpoints exposed as MCP tools
- Built with the official MCP TypeScript SDK
- Async/await support with modern TypeScript
- Automatic SOAP envelope construction and XML parsing
- Secure credential handling via environment variables

## Installation

### Quick Install (Cursor IDE)

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en-US/install-mcp?name=studentvue&config=eyJjb21tYW5kIjoibm9kZSIsImFyZ3MiOlsiL3BhdGgvdG8vc3R1ZGVudHZ1ZS9wYWNrYWdlcy9zdHVkZW50dnVlLW1jcC9kaXN0L2luZGV4LmpzIl0sImVudiI6eyJTVFVERU5UVlVFX1BPUlRBTCI6Imh0dHBzOi8veW91ci1kaXN0cmljdC5lZHVwb2ludC5jb20iLCJTVFVERU5UVlVFX1VTRVJOQU1FIjoieW91cl91c2VybmFtZSIsIlNUVURFTlRWVUVfUEFTU1dPUkQiOiJ5b3VyX3Bhc3N3b3JkIn19)

Click the button above to install in Cursor, then update with your credentials.

**For complete installation instructions, see [INSTALL.md](./INSTALL.md)**

### Manual Setup

#### Prerequisites

- Node.js 18.0.0 or higher
- Bun (for development and building)
- Valid StudentVue account credentials

#### Build Steps

1. Install dependencies:

```bash
cd packages/studentvue-mcp
bun install
```

2. Build the server:

```bash
bun run build
```

3. Configure with your AI application (see [INSTALL.md](./INSTALL.md) for details)

## Usage

### Configuration

See [INSTALL.md](./INSTALL.md) for complete installation and configuration instructions for:
- Claude Desktop
- Cursor IDE
- Other MCP-compatible applications

### Running in Development

```bash
bun run dev
```

## Available Tools

### Student Information
- `get_student_info` - Retrieve student profile including name, grade, school, and contact information
- `get_school_info` - Retrieve school details including principal, address, and contact information

### Academic Records
- `get_gradebook` - Retrieve current grades, assignments, and course information
- `get_class_schedule` - Retrieve class schedule with periods, teachers, and room assignments
- `get_calendar` - Retrieve calendar events and upcoming assignments for a specific date
- `get_class_notes` - Retrieve homework notes (district-dependent feature)

### Attendance
- `get_attendance` - Retrieve attendance records including absences, tardies, and reasons

### Communication
- `get_messages` - Retrieve inbox messages from teachers and administrators
- `mark_message_read` - Mark a specific message as read

### Documents
- `list_documents` - List all available documents
- `get_document` - Download a specific document by GUID
- `list_report_cards` - List available report cards by grading period
- `get_report_card` - Download a specific report card

### Health Records
- `get_student_health_info` - Retrieve student health information

### Utilities
- `get_districts_by_zip` - Search for school districts by ZIP code

## Development

### Build Commands

```bash
bun run build       # Build the TypeScript project
bun run dev         # Run in development mode
bun run format      # Format code with Prettier
bun run lint        # Run ESLint
bun run typecheck   # Run TypeScript type checking
bun test            # Run tests
bun run quality-gates # Run all quality checks
```

## Example Usage with AI

Once configured, you can ask Claude:

- "What are my current grades?"
- "Show me my attendance record"
- "What messages do I have from teachers?"
- "What's on my calendar for today?"
- "Show me my class schedule"

The MCP server will automatically call the appropriate StudentVue API endpoints and return formatted data.

## Security Considerations

**Important:** Never commit credentials to version control. Always use environment variables or secure credential management. The `.env` file is automatically gitignored.

## Architecture

This MCP server:
1. Implements the StudentVue SOAP API protocol
2. Exposes API methods as MCP tools
3. Communicates with AI applications via stdio
4. Returns XML data that AI models can parse and present to users

## License

MIT

## Related Projects

- [StudentVue Rust API](../studentvue-api/) - The Rust implementation of the StudentVue API client

