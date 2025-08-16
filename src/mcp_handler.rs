use anyhow::Result;
use rmcp::{
    ErrorData as McpError, ServerHandler,
    model::{
        CallToolRequestMethod, CallToolRequestParam, CallToolResult, Content, Implementation,
        InitializeRequestParam, InitializeResult, ListToolsResult, PaginatedRequestParam,
        ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
    },
    service::{RequestContext, RoleServer},
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

    /// Handle the list_tasks tool call
    async fn handle_list_tasks(
        &self,
        arguments: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<CallToolResult, McpError> {
        let task_collection =
            self.task_service.load_tasks().await.map_err(|e| {
                McpError::internal_error(format!("Failed to load tasks: {}", e), None)
            })?;

        let filters: HashMap<String, String> = arguments
            .unwrap_or_default()
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
            .collect();

        let filtered_tasks = self
            .task_service
            .filter_tasks(&task_collection.tasks, &filters);

        let response = serde_json::json!({
            "count": filtered_tasks.len(),
            "tasks": filtered_tasks,
            "filters_applied": filters
        });

        let response_text = serde_json::to_string_pretty(&response).map_err(|e| {
            McpError::internal_error(format!("Failed to serialize response: {}", e), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(response_text)]))
    }

    /// Handle the get_task tool call
    async fn handle_get_task(
        &self,
        arguments: serde_json::Map<String, serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let task_id = arguments
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: id", None))?;

        let task = self
            .task_service
            .find_task_by_id(task_id)
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to load tasks: {}", e), None))?
            .ok_or_else(|| {
                McpError::invalid_params(format!("Task not found: {}", task_id), None)
            })?;

        let response = serde_json::json!({
            "task": task
        });

        let response_text = serde_json::to_string_pretty(&response).map_err(|e| {
            McpError::internal_error(format!("Failed to serialize task: {}", e), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(response_text)]))
    }

    /// Handle the task_stats tool call
    async fn handle_task_stats(&self) -> Result<CallToolResult, McpError> {
        let stats = self.task_service.get_task_statistics().await.map_err(|e| {
            McpError::internal_error(format!("Failed to get task statistics: {}", e), None)
        })?;

        let response = serde_json::json!({
            "total_tasks": stats.total_tasks,
            "status_counts": stats.status_counts,
            "priority_counts": stats.priority_counts
        });

        let response_text = serde_json::to_string_pretty(&response).map_err(|e| {
            McpError::internal_error(format!("Failed to serialize statistics: {}", e), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(response_text)]))
    }
}

impl ServerHandler for TaskMcpHandler {
    fn get_info(&self) -> ServerInfo {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
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
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let tools = vec![
            Tool {
                name: "list_tasks".into(),
                description: Some(
                    "List all tasks, optionally filtered by status, priority, assignee, or tag"
                        .into(),
                ),
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
                output_schema: None,
                annotations: None,
            },
            Tool {
                name: "get_task".into(),
                description: Some("Get detailed information about a specific task by ID".into()),
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
                output_schema: None,
                annotations: None,
            },
            Tool {
                name: "task_stats".into(),
                description: Some(
                    "Get statistics about tasks (counts by status, priority, etc.)".into(),
                ),
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
                output_schema: None,
                annotations: None,
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
