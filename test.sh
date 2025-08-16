#!/usr/bin/env zsh
set -euo pipefail

# ---- Build the server first ----
echo "Building the MCP server..."
cargo build --bin mcp_todo_task
echo "Server built successfully."

# --- Server command ---
server="target/debug/mcp_todo_task"

echo "Starting MCP server test..."

# ---- Test the server with simple JSON lines ----
(
  # Send initialize request
  echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"TerminalClient","version":"0.1"}}}'
  
  # Send notifications/initialized
  echo '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  
  # List available tools
  echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'
  
  # Call the list_tasks tool
  echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list_tasks","arguments":{}}}'
  
  # Give the server time to process
  sleep 1
) | $server | while IFS= read -r line; do
  # Check if this is a JSON response (starts with {)
  if [[ "$line" =~ ^\{.*\}$ ]]; then
    echo "Response:"
    echo "$line" | jq . 2>/dev/null || echo "$line"
    echo "---"
  fi
done

echo "Test completed."
