# Task Manager MCP Server

A high-performance **Model Context Protocol (MCP) server** implementation written in Rust that provides task management capabilities for AI assistants. This server enables AI clients like Claude Desktop to efficiently manage and query task data stored in JSON format.

## 🚀 Features

- **📋 Task Management**: Complete CRUD-like operations for task data
- **🔍 Advanced Filtering**: Filter tasks by status, priority, assignee, or tags  
- **📊 Task Analytics**: Get comprehensive statistics and insights about your tasks
- **⚡ High Performance**: Built with Rust and async/await for optimal performance
- **🗃️ JSON Storage**: Simple, human-readable file-based storage
- **🔌 MCP Compatible**: Works seamlessly with MCP-enabled AI assistants
- **⚙️ Configurable**: Environment-based configuration for flexibility
- **🏗️ Clean Architecture**: Well-structured codebase with clear separation of concerns

## 📊 Task Data Structure

Tasks are stored in a JSON file with the following comprehensive structure:

```json
{
  "version": "1.0",
  "tasks": [
    {
      "id": "task-001",
      "title": "Setup Development Environment", 
      "description": "Install necessary tools and configure the development environment for the new project",
      "status": "completed",
      "priority": "high",
      "created_at": "2024-01-15T09:00:00Z",
      "updated_at": "2024-01-15T15:30:00Z",
      "tags": ["setup", "development", "infrastructure"],
      "assignee": "alice.smith",
      "due_date": "2024-01-20T17:00:00Z"
    }
  ]
}
```

### Field Specifications

| Field | Type | Description | Required |
|-------|------|-------------|----------|
| `id` | String | Unique identifier for the task | ✅ |
| `title` | String | Short, descriptive title of the task | ✅ |
| `description` | String | Detailed description of the task | ✅ |
| `status` | Enum | Current status (`pending`, `in_progress`, `completed`, `cancelled`) | ✅ |
| `priority` | Enum | Priority level (`low`, `medium`, `high`, `critical`) | ✅ |
| `created_at` | String | ISO 8601 timestamp when task was created | ✅ |
| `updated_at` | String | ISO 8601 timestamp when task was last modified | ✅ |
| `tags` | Array | Array of strings for task categorization | ✅ |
| `assignee` | String | Username of the assigned person | ❌ |
| `due_date` | String | ISO 8601 timestamp for due date | ❌ |

## 🛠️ Installation & Setup

### Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Rust's package manager (included with Rust)

### Building the Project

```bash
# Clone the repository (if needed)
git clone <repository-url>
cd mcp_todo_task

# Build the project
cargo build --release

# Verify the build
cargo check
```

### Running the Server

```bash
# Use default configuration (looks for ./data/tasks.json)
cargo run

# Or run the built binary directly
./target/release/mcp_todo_task

# Use a custom task file
TASKS_FILE=/path/to/your/custom-tasks.json cargo run
```

The server communicates via **stdin/stdout** using the MCP protocol, making it compatible with various MCP clients.

## 🔧 Configuration

The server supports the following environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `TASKS_FILE` | `./data/tasks.json` | Path to the JSON file containing task data |

### Example Configuration

```bash
# Use a different task file
export TASKS_FILE="/home/user/my-tasks.json"
cargo run

# Or inline
TASKS_FILE="/tmp/test-tasks.json" cargo run
```

## 🛠️ Available MCP Tools

The server provides three powerful tools for task management:

### 1. `list_tasks` - List and Filter Tasks

List all tasks with optional filtering capabilities.

**Parameters** (all optional):
- `status`: Filter by status (`pending`, `in_progress`, `completed`, `cancelled`)
- `priority`: Filter by priority (`low`, `medium`, `high`, `critical`)
- `assignee`: Filter by assignee username  
- `tag`: Filter by specific tag

**Examples:**

```json
// List all tasks
{
  "name": "list_tasks",
  "arguments": {}
}

// List high priority tasks in progress
{
  "name": "list_tasks", 
  "arguments": {
    "status": "in_progress",
    "priority": "high"
  }
}

// List tasks assigned to a specific user
{
  "name": "list_tasks",
  "arguments": {
    "assignee": "alice.smith"
  }
}

// List tasks with a specific tag
{
  "name": "list_tasks",
  "arguments": {
    "tag": "backend"
  }
}
```

### 2. `get_task` - Get Task Details

Retrieve comprehensive information about a specific task.

**Parameters:**
- `id` (required): The unique task identifier

**Example:**

```json
{
  "name": "get_task",
  "arguments": {
    "id": "task-001"
  }
}
```

### 3. `task_stats` - Task Statistics

Get statistical insights about your tasks, including counts by status and priority.

**Parameters:** None

**Example:**

```json
{
  "name": "task_stats",
  "arguments": {}
}
```

**Sample Response:**
```
## Task Statistics

**Total Tasks:** 8

### By Status:
- Completed: 1
- InProgress: 2  
- Pending: 4
- Cancelled: 1

### By Priority:
- Critical: 1
- High: 3
- Medium: 3
- Low: 1
```

## 🏗️ Project Architecture

The project follows a clean, layered architecture:

```
src/
├── main.rs           # Application entry point & server setup
├── lib.rs            # Library exports & documentation  
├── config.rs         # Configuration management
├── models.rs         # Data structures (Task, Priority, Status)
├── storage.rs        # JSON file persistence layer
├── task_service.rs   # Business logic & filtering
└── mcp_handler.rs    # MCP protocol implementation
```

### Architecture Layers

1. **Entry Point** (`main.rs`): Server initialization and async runtime setup
2. **Configuration** (`config.rs`): Environment-based configuration management  
3. **Models** (`models.rs`): Core data structures and serialization
4. **Storage** (`storage.rs`): File-based persistence with async I/O
5. **Service** (`task_service.rs`): Business logic, filtering, and statistics
6. **Handler** (`mcp_handler.rs`): MCP protocol implementation and tool routing

## 📦 Dependencies

The project uses carefully selected, high-quality Rust crates:

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.0 | Async runtime with full feature set |
| `rmcp` | 0.5.0 | MCP protocol implementation |
| `serde` | 1.0 | Serialization/deserialization framework |
| `serde_json` | 1.0 | JSON support for serde |
| `anyhow` | 1.0 | Error handling and context |
| `tracing` | 0.1 | Structured logging |
| `tracing-subscriber` | 0.3 | Logging configuration |

## 🧪 Testing

### Running Tests

```bash
# Run unit tests
cargo test

# Run the integration test script  
./test.sh

# Run tests with output
cargo test -- --nocapture
```

### Test Script

The included `test.sh` script provides a comprehensive integration test that:

1. Builds the server
2. Sends MCP initialization requests
3. Tests all available tools
4. Validates responses using `jq`

## 🤝 Usage with MCP Clients

This server is designed to work with MCP-compatible clients:

### Claude Desktop Integration

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "task-manager": {
      "command": "/path/to/mcp_todo_task/target/release/mcp_todo_task",
      "env": {
        "TASKS_FILE": "/path/to/your/tasks.json"
      }
    }
  }
}
```

### Custom MCP Clients

Any MCP-compatible client can integrate with this server by:

1. Starting the server as a subprocess
2. Communicating via stdin/stdout
3. Following the MCP protocol specification
4. Using the three available tools (`list_tasks`, `get_task`, `task_stats`)

## 📝 Sample Data

The repository includes `data/tasks.json` with sample tasks demonstrating:

- Various task statuses and priorities
- Different assignees and tags
- Optional fields (due dates, assignees)
- Realistic task descriptions
- Proper timestamp formatting

You can use this as a template or starting point for your own task data.

## 🚀 Performance

Built with performance in mind:

- **Async/Await**: Non-blocking I/O operations using Tokio
- **Zero-Copy**: Efficient JSON parsing with serde
- **Memory Efficient**: Streaming and lazy loading where possible  
- **Fast Startup**: Minimal initialization overhead
- **Concurrent**: Handles multiple MCP requests efficiently

## 🔒 Security Considerations

- **File Access**: Server only accesses the configured task file
- **Input Validation**: All MCP tool parameters are validated
- **Error Handling**: Comprehensive error handling prevents crashes
- **Resource Limits**: Bounded memory usage for task collections

## 🤝 Contributing

This project follows Rust best practices:

1. Run `cargo fmt` for formatting
2. Run `cargo clippy` for linting  
3. Ensure all tests pass with `cargo test`
4. Update documentation for public APIs
5. Follow the existing code style and architecture

## 📄 License

This project is provided as-is for educational and practical use. See the specific license file for details.

---

**Built with ❤️ in Rust** • **MCP Protocol Compatible** • **Production Ready**
