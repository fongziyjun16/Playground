# MCP Server Demo

## Requirements

- VSCode
- VSCode Plugin Cline
  - In Settings Choose `OpenAI` as API Provider & Input OpenAI API Key
- NodeJS

## Demo Code

```typescript
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { SSEServerTransport } from "@modelcontextprotocol/sdk/server/sse.js";
import express from "express";
import { z } from "zod";

// Create an MCP server
const server = new McpServer({
  name: "Demo",
  version: "1.0.0"
}, {
  capabilities: {}
});


// Add an addition tool
server.tool("add",
  'Add two numbers',
  { a: z.number(), b: z.number() },
  async ({ a, b }) => {
    return {
      content: [{ type: "text", text: String(a + b) + " from mcp demo server" }]
    }
  }
);

async function main() {
  // stdio
  const transport = new StdioServerTransport();
  await server.connect(transport);

  // sse
  // const app = express();
  // let transport: SSEServerTransport;
  // app.get("/sse", async (req, res) => {
  //   console.log("Received connection");
  //   transport = new SSEServerTransport("/messages", res);
  //   await server.connect(transport);
  // })
  // app.post("/messages", async (req, res) => {
  //   console.log("Received message");
  //   await transport.handlePostMessage(req, res);
  // })
  // app.listen(3060, () => {
  //   console.log("Server is running.")
  // });
}

main()

// cline_mcp_settings.json

// sse
// {
//   "mcpServers": {
//     "mcp_demo_server": {
//       "url": "http://localhost:3060/sse",
//       "disabled": false,
//       "autoApprove": [
//         "add"
//       ]
//     }
//   }
// }

// stdio
// {
//   "mcpServers": {
//     "mcp_demo_server": {
//       "command": "node",
//       "args": ["$HOME/proj/build/index.js"],
//       "autoApprove": [
//         "add"
//       ]
//     }
//   }
// }
```

- Tool Auto Approved

  - add configuration to `cline_mcp_settings.json`

  - in cline task tab
    - Check `Use MCP Servers`
    - Check `Auto-approve: MCP`