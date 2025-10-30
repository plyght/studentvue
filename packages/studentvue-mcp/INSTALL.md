# StudentVue MCP Server Installation

## Quick Install via Cursor

Click the button below to install the StudentVue MCP server in Cursor:

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en-US/install-mcp?name=studentvue&config=eyJjb21tYW5kIjoibm9kZSIsImFyZ3MiOlsiL3BhdGgvdG8vc3R1ZGVudHZ1ZS9wYWNrYWdlcy9zdHVkZW50dnVlLW1jcC9kaXN0L2luZGV4LmpzIl0sImVudiI6eyJTVFVERU5UVlVFX1BPUlRBTCI6Imh0dHBzOi8veW91ci1kaXN0cmljdC5lZHVwb2ludC5jb20iLCJTVFVERU5UVlVFX1VTRVJOQU1FIjoieW91cl91c2VybmFtZSIsIlNUVURFTlRWVUVfUEFTU1dPUkQiOiJ5b3VyX3Bhc3N3b3JkIn19)

**Important:** After installation, you must update the configuration with your actual credentials and file path.

## Manual Installation Methods

### Method 1: Claude Desktop Configuration

Add to your Claude Desktop configuration file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "studentvue": {
      "command": "node",
      "args": ["/absolute/path/to/studentvue/packages/studentvue-mcp/dist/index.js"],
      "env": {
        "STUDENTVUE_PORTAL": "https://your-district.edupoint.com",
        "STUDENTVUE_USERNAME": "your_username",
        "STUDENTVUE_PASSWORD": "your_password"
      }
    }
  }
}
```

### Method 2: Using Bun Runtime

```json
{
  "mcpServers": {
    "studentvue": {
      "command": "bun",
      "args": ["run", "/absolute/path/to/studentvue/packages/studentvue-mcp/src/index.ts"],
      "env": {
        "STUDENTVUE_PORTAL": "https://your-district.edupoint.com",
        "STUDENTVUE_USERNAME": "your_username",
        "STUDENTVUE_PASSWORD": "your_password"
      }
    }
  }
}
```

### Method 3: Cursor IDE Configuration

1. Open Cursor Settings
2. Navigate to Features > Model Context Protocol
3. Click "Add MCP Server"
4. Enter the server name: `studentvue`
5. Enter the configuration:

```json
{
  "command": "node",
  "args": ["/absolute/path/to/studentvue/packages/studentvue-mcp/dist/index.js"],
  "env": {
    "STUDENTVUE_PORTAL": "https://your-district.edupoint.com",
    "STUDENTVUE_USERNAME": "your_username",
    "STUDENTVUE_PASSWORD": "your_password"
  }
}
```

## Configuration Steps

### 1. Build the Server

```bash
cd packages/studentvue-mcp
bun install
bun run build
```

### 2. Find Your District Portal

Use the district lookup tool to find your portal URL:

**Using the Rust API:**
```bash
cd packages/studentvue-api
cargo run --example basic_usage
```

**Or ask Claude after installation:**
```
Find StudentVue districts for ZIP code [your zip]
```

Portal URLs typically follow: `https://[district-name].edupoint.com`

### 3. Update Configuration

Replace the placeholder values:
- `STUDENTVUE_PORTAL`: Your district's portal URL
- `STUDENTVUE_USERNAME`: Your student ID or username
- `STUDENTVUE_PASSWORD`: Your StudentVue password
- File path: Absolute path to `dist/index.js`

### 4. Restart Your Application

- **Claude Desktop:** Completely quit and restart the application
- **Cursor:** Restart the IDE or reload the window

## Verification

After installation, test the server by asking your AI assistant:

```
What are my current grades?
```

If configured correctly, you should receive your StudentVue grade information.

## Troubleshooting

### Server Not Responding

1. Verify the build completed successfully:
   ```bash
   ls -la /path/to/studentvue/packages/studentvue-mcp/dist/index.js
   ```

2. Check credentials are correct in the configuration

3. Verify Node.js is installed and accessible:
   ```bash
   node --version
   ```

### Authentication Errors

1. Verify your StudentVue credentials by logging into the web portal
2. Ensure the portal URL is correct (no trailing slash)
3. Check that environment variables are set in the configuration

### Path Issues

1. Use absolute paths, not relative paths
2. Expand `~` to full home directory path
3. On Windows, use forward slashes or escaped backslashes

### Claude Desktop Logs

Check logs for errors:

**macOS:**
```bash
tail -f ~/Library/Logs/Claude/mcp*.log
```

**Windows:**
```powershell
Get-Content "$env:APPDATA\Claude\Logs\mcp*.log" -Wait
```

## Security Notes

- Never commit your configuration with real credentials to version control
- Store credentials securely using environment variables
- Consider using a password manager to generate and store credentials
- The MCP server runs locally and does not send data to third parties

## Generating Custom Install Links

To create a custom install link for your institution:

1. Create your configuration JSON (server config only, without the server name wrapper):
```json
{
  "command": "node",
  "args": ["/path/to/dist/index.js"],
  "env": {
    "STUDENTVUE_PORTAL": "https://your-district.edupoint.com",
    "STUDENTVUE_USERNAME": "",
    "STUDENTVUE_PASSWORD": ""
  }
}
```

2. Base64 encode the JSON:
```bash
echo -n '{"command":"node","args":["/path/to/dist/index.js"],"env":{"STUDENTVUE_PORTAL":"https://your-district.edupoint.com","STUDENTVUE_USERNAME":"","STUDENTVUE_PASSWORD":""}}' | base64
```

3. Create the deeplink:
```
https://cursor.com/en-US/install-mcp?name=studentvue&config=[BASE64_OUTPUT]
```

4. Create the markdown button:
```markdown
[![Install MCP Server](https://cursor.com/deeplink/mcp-install-dark.svg)](https://cursor.com/en-US/install-mcp?name=studentvue&config=[BASE64_OUTPUT])
```

## Next Steps

After successful installation:

1. Review available tools in [README.md](./README.md)
2. Test each feature with your AI assistant
3. Check out example queries in the documentation
4. Report any issues on GitHub

## Support

- [Main Documentation](./README.md)
- [API Reference](../studentvue-api/README.md)
- [Monorepo Guide](../../docs/MONOREPO_GUIDE.md)
- [MCP Protocol Documentation](https://modelcontextprotocol.io/)

