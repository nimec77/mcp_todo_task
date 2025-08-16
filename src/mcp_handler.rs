use anyhow::Result;
use rmcp::{
    model::{
        CallToolRequestParam, CallToolResult, Content, InitializeRequestParam, InitializeResult,
        ListToolsResult, PaginatedRequestParam, ServerCapabilities, ServerInfo, Tool,
        Implementation, ProtocolVersion, CallToolRequestMethod,
    },
    service::{RequestContext, RoleServer},
    ServerHandler, Error as McpError,
};
use std::{collections::HashMap, sync::Arc};
use tracing::info;

use crate::task_service::TaskService;

/// MCP server handler that manages tasks
#[derive(Debug, Clone)]
pub struct TaskMcpHandler {
    task_service: TaskService,
}

impl TaskMcpHandler {
    /// Create a new MCP handler with the given task service
    pub fn new(task_service: TaskService) -> Self {
        Self { task_service }
    }

    /// Format a list of tasks as a human-readable string
    fn format_task_list(&self, tasks: &[crate::models::Task]) -> String {
        tasks
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
            .join("\n\n")
    }

    /// Handle the list_tasks tool call
    async fn handle_list_tasks(&self, arguments: Option<serde_json::Map<String, serde_json::Value>>) -> Result<CallToolResult, McpError> {
        let task_collection = self
            .task_service
            .load_tasks()
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?;

        let filters: HashMap<String, String> = arguments
            .unwrap_or_default()
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect();

        let filtered_tasks = self.task_service.filter_tasks(&task_collection.tasks, &filters);

        let summary = if filtered_tasks.is_empty() {
            "No tasks found with the specified filters.".to_string()
        } else {
            let task_list = self.format_task_list(&filtered_tasks);
            format!(
                "Found {} task(s):\n\n{}",
                filtered_tasks.len(),
                task_list
            )
        };

        Ok(CallToolResult::success(vec![Content::text(summary)]))
    }

    /// Handle the get_task tool call
    async fn handle_get_task(&self, arguments: serde_json::Map<String, serde_json::Value>) -> Result<CallToolResult, McpError> {
        let task_id = arguments
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: id", None))?;

        let task = self
            .task_service
            .find_task_by_id(task_id)
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?
            .ok_or_else(|| McpError::invalid_params(format!("Task not found: {}", task_id), None))?;

        let task_details = serde_json::to_string_pretty(&task)
            .map_err(|e| McpError::internal_error(format!("Failed to serialize task: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Task Details:\n```json\n{}\n```",
            task_details
        ))]))
    }

    /// Handle the task_stats tool call
    async fn handle_task_stats(&self) -> Result<CallToolResult, McpError> {
        let stats = self
            .task_service
            .get_task_statistics()
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to get task statistics: {}", e), None))?;

        let formatted_stats = stats.format_stats();
        Ok(CallToolResult::success(vec![Content::text(formatted_stats)]))
    }
}

impl ServerHandler for TaskMcpHandler {
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
            "list_tasks" => self.handle_list_tasks(request.arguments).await,
            "get_task" => {
                let arguments = request.arguments.unwrap_or_default();
                self.handle_get_task(arguments).await
            }
            "task_stats" => self.handle_task_stats().await,
            _ => Err(McpError::method_not_found::<CallToolRequestMethod>()),
        }
    }
}
