# Task Manager MCP Server

A Model Context Protocol (MCP) server implementation that reads task data from a JSON file and provides task management tools for AI assistants.

## Features

- **List Tasks**: Filter and view tasks by status, priority, assignee, or tags
- **Get Task Details**: Retrieve detailed information about a specific task
- **Task Statistics**: Get overview statistics about tasks by status and priority
- **JSON File Storage**: Simple file-based storage for easy editing and backup

## Task Data Format

Tasks are stored in a JSON file with the following structure:

```json
{
  "version": "1.0",
  "tasks": [
    {
      "id": "task-001",
      "title": "Task Title",
      "description": "Task description here",
      "status": "pending|in_progress|completed|cancelled",
      "priority": "low|medium|high|critical",
      "created_at": "2024-01-15T09:00:00Z",
      "updated_at": "2024-01-15T15:30:00Z",
      "tags": ["tag1", "tag2"],
      "assignee": "username",
      "due_date": "2024-01-20T17:00:00Z"
    }
  ]
}
```

### Field Descriptions

- `id`: Unique identifier for the task
- `title`: Short title/summary of the task
- `description`: Detailed description of the task
- `status`: Current status (pending, in_progress, completed, cancelled)
- `priority`: Task priority level (low, medium, high, critical)
- `created_at`: ISO 8601 timestamp when task was created
- `updated_at`: ISO 8601 timestamp when task was last modified
- `tags`: Array of tags for categorization
- `assignee`: Username of the person assigned (optional)
- `due_date`: Due date for the task (optional)

## Building and Running

### Prerequisites

- Rust 1.70+
- Tokio runtime

### Build

```bash
cargo build --release
```

### Run

```bash
# Use default tasks.json file in current directory
cargo run

# Use custom task file
TASKS_FILE=/path/to/your/tasks.json cargo run
```

The server communicates via stdin/stdout using the MCP protocol.

## Available Tools

### 1. list_tasks

List all tasks with optional filtering.

**Parameters:**
- `status` (optional): Filter by status (pending, in_progress, completed, cancelled)
- `priority` (optional): Filter by priority (low, medium, high, critical)  
- `assignee` (optional): Filter by assignee username
- `tag` (optional): Filter by tag name

**Example:**
```json
{
  "name": "list_tasks",
  "arguments": {
    "status": "in_progress",
    "priority": "high"
  }
}
```

### 2. get_task

Get detailed information about a specific task.

**Parameters:**
- `id` (required): The task ID

**Example:**
```json
{
  "name": "get_task", 
  "arguments": {
    "id": "task-001"
  }
}
```

### 3. task_stats

Get statistics about tasks grouped by status and priority.

**Parameters:** None

**Example:**
```json
{
  "name": "task_stats",
  "arguments": {}
}
```

## Configuration

The server can be configured using environment variables:

- `TASKS_FILE`: Path to the JSON file containing task data (default: `tasks.json`)

## Dependencies

- `tokio`: Async runtime
- `rmcp`: MCP protocol implementation
- `serde`: Serialization/deserialization
- `serde_json`: JSON support
- `anyhow`: Error handling
- `tracing`: Logging

## Usage with MCP Clients

This server is designed to work with MCP-compatible clients like:

- Claude Desktop
- Custom MCP implementations
- AI assistants supporting the MCP protocol

Configure your MCP client to run this server as a subprocess for task management capabilities.

## Sample Data

The repository includes a `tasks.json` file with sample tasks demonstrating the data format. You can modify this file to add your own tasks or create a new file and specify its path via the `TASKS_FILE` environment variable.

## License

This project is provided as-is for educational and practical use.
