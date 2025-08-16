use anyhow::Result;
use rmcp::{
    model::{
        CallToolRequestParam, CallToolResult, Content, InitializeRequestParam, InitializeResult,
        ListToolsResult, PaginatedRequestParam, ServerCapabilities, ServerInfo, Tool,
        Implementation, ProtocolVersion, CallToolRequestMethod,
    },
    service::{RequestContext, RoleServer, ServiceExt},
    ServerHandler, Error as McpError,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;
use tracing::info;

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

/// Individual task structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub assignee: Option<String>,
    pub due_date: Option<String>,
}

/// Container for all tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCollection {
    pub tasks: Vec<Task>,
    pub version: String,
}

/// Our MCP server handler that manages tasks
#[derive(Debug, Clone)]
pub struct TaskServer {
    tasks_file_path: PathBuf,
}

impl TaskServer {
    pub fn new(tasks_file_path: PathBuf) -> Self {
        Self { tasks_file_path }
    }

    /// Load tasks from the JSON file
    async fn load_tasks(&self) -> Result<TaskCollection> {
        if !self.tasks_file_path.exists() {
            // Return empty collection if file doesn't exist
            return Ok(TaskCollection {
                tasks: Vec::new(),
                version: "1.0".to_string(),
            });
        }

        let content = fs::read_to_string(&self.tasks_file_path).await?;
        let tasks: TaskCollection = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    /// Save tasks to the JSON file
    #[allow(dead_code)]
    async fn save_tasks(&self, tasks: &TaskCollection) -> Result<()> {
        let content = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.tasks_file_path, content).await?;
        Ok(())
    }

    /// Filter tasks based on criteria
    fn filter_tasks(&self, tasks: &[Task], filters: &HashMap<String, String>) -> Vec<Task> {
        tasks
            .iter()
            .filter(|task| {
                for (key, value) in filters {
                    match key.as_str() {
                        "status" => {
                            let status_match = match value.as_str() {
                                "pending" => task.status == TaskStatus::Pending,
                                "in_progress" => task.status == TaskStatus::InProgress,
                                "completed" => task.status == TaskStatus::Completed,
                                "cancelled" => task.status == TaskStatus::Cancelled,
                                _ => false,
                            };
                            if !status_match {
                                return false;
                            }
                        }
                        "priority" => {
                            let priority_match = match value.as_str() {
                                "low" => task.priority == Priority::Low,
                                "medium" => task.priority == Priority::Medium,
                                "high" => task.priority == Priority::High,
                                "critical" => task.priority == Priority::Critical,
                                _ => false,
                            };
                            if !priority_match {
                                return false;
                            }
                        }
                        "assignee" => {
                            if task.assignee.as_ref() != Some(value) {
                                return false;
                            }
                        }
                        "tag" => {
                            if !task.tags.contains(value) {
                                return false;
                            }
                        }
                        _ => {} // Ignore unknown filters
                    }
                }
                true
            })
            .cloned()
            .collect()
    }
}

impl ServerHandler for TaskServer {
    fn get_info(&self) -> ServerInfo {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: None,
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        info!("Task Manager MCP server initialized");
        Ok(InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: self.get_info().capabilities,
            server_info: Implementation::from_build_env(),
            instructions: None,
        })
    }

    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let tools = vec![
            Tool {
                name: "list_tasks".into(),
                description: "List all tasks, optionally filtered by status, priority, assignee, or tag".into(),
                input_schema: Arc::new({
                    let schema = serde_json::json!({
                        "type": "object",
                        "properties": {
                            "status": {
                                "type": "string",
                                "enum": ["pending", "in_progress", "completed", "cancelled"],
                                "description": "Filter tasks by status"
                            },
                            "priority": {
                                "type": "string",
                                "enum": ["low", "medium", "high", "critical"],
                                "description": "Filter tasks by priority"
                            },
                            "assignee": {
                                "type": "string",
                                "description": "Filter tasks by assignee"
                            },
                            "tag": {
                                "type": "string",
                                "description": "Filter tasks by tag"
                            }
                        },
                        "additionalProperties": false
                    });
                    match schema {
                        serde_json::Value::Object(map) => map,
                        _ => panic!("Schema must be an object"),
                    }
                }),
            },
            Tool {
                name: "get_task".into(),
                description: "Get detailed information about a specific task by ID".into(),
                input_schema: Arc::new({
                    let schema = serde_json::json!({
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "The task ID"
                            }
                        },
                        "required": ["id"],
                        "additionalProperties": false
                    });
                    match schema {
                        serde_json::Value::Object(map) => map,
                        _ => panic!("Schema must be an object"),
                    }
                }),
            },
            Tool {
                name: "task_stats".into(),
                description: "Get statistics about tasks (counts by status, priority, etc.)".into(),
                input_schema: Arc::new({
                    let schema = serde_json::json!({
                        "type": "object",
                        "properties": {},
                        "additionalProperties": false
                    });
                    match schema {
                        serde_json::Value::Object(map) => map,
                        _ => panic!("Schema must be an object"),
                    }
                }),
            },
        ];

        Ok(ListToolsResult { 
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "list_tasks" => {
                let task_collection = self
                    .load_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?;

                let filters: HashMap<String, String> = request
                    .arguments
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect();

                let filtered_tasks = self.filter_tasks(&task_collection.tasks, &filters);

                let task_list = filtered_tasks
                    .iter()
                    .map(|task| {
                        format!(
                            "- **{}** (ID: {}) - Status: {:?}, Priority: {:?}\n  Description: {}\n  Tags: [{}]{}{}",
                            task.title,
                            task.id,
                            task.status,
                            task.priority,
                            task.description,
                            task.tags.join(", "),
                            task.assignee.as_ref().map(|a| format!("\n  Assignee: {}", a)).unwrap_or_default(),
                            task.due_date.as_ref().map(|d| format!("\n  Due: {}", d)).unwrap_or_default()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");

                let summary = if filtered_tasks.is_empty() {
                    "No tasks found with the specified filters.".to_string()
                } else {
                    format!(
                        "Found {} task(s):\n\n{}",
                        filtered_tasks.len(),
                        task_list
                    )
                };

                Ok(CallToolResult::success(vec![Content::text(summary)]))
            }
            "get_task" => {
                let arguments = request.arguments.unwrap_or_default();
                let task_id = arguments
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| McpError::invalid_params("Missing required parameter: id", None))?;

                let task_collection = self
                    .load_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?;

                let task = task_collection
                    .tasks
                    .iter()
                    .find(|t| t.id == task_id)
                    .ok_or_else(|| McpError::invalid_params(format!("Task not found: {}", task_id), None))?;

                let task_details = serde_json::to_string_pretty(task)
                    .map_err(|e| McpError::internal_error(format!("Failed to serialize task: {}", e), None))?;

                Ok(CallToolResult::success(vec![Content::text(format!("Task Details:\n```json\n{}\n```", task_details))]))
            }
            "task_stats" => {
                let task_collection = self
                    .load_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?;

                let total_tasks = task_collection.tasks.len();
                let mut status_counts = HashMap::new();
                let mut priority_counts = HashMap::new();

                for task in &task_collection.tasks {
                    *status_counts.entry(format!("{:?}", task.status)).or_insert(0) += 1;
                    *priority_counts.entry(format!("{:?}", task.priority)).or_insert(0) += 1;
                }

                let stats = format!(
                    "## Task Statistics\n\n**Total Tasks:** {}\n\n### By Status:\n{}\n\n### By Priority:\n{}",
                    total_tasks,
                    status_counts
                        .iter()
                        .map(|(status, count)| format!("- {}: {}", status, count))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    priority_counts
                        .iter()
                        .map(|(priority, count)| format!("- {}: {}", priority, count))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                Ok(CallToolResult::success(vec![Content::text(stats)]))
            }
            _ => Err(McpError::method_not_found::<CallToolRequestMethod>()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Task Manager MCP Server");

    // Set up the task file path - defaulting to ./data/tasks.json
    let tasks_file_path = std::env::var("TASKS_FILE")
        .unwrap_or_else(|_| "./data/tasks.json".to_string())
        .into();

    // Create the server handler
    let server = TaskServer::new(tasks_file_path);

    // Set up transport - using stdio for MCP communication
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    info!("Task Manager MCP Server starting...");

    // Start the server
    let _running_server = server.serve(transport).await?;

    info!("Task Manager MCP Server is running");

    // Keep the server running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Task Manager MCP Server");

    Ok(())
}
