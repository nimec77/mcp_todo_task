//! Task Manager MCP Server Library
//!
//! This library provides a Model Context Protocol (MCP) server for task management.
//! It supports reading and querying task data from JSON files with filtering capabilities.
//!
//! ## Features
//!
//! - **Task Management**: Load, filter, and query tasks
//! - **MCP Protocol**: Full MCP server implementation
//! - **JSON Storage**: Simple file-based task persistence
//! - **Flexible Configuration**: Environment-based configuration
//!
//! ## Example
//!
//! ```rust,no_run
//! use mcp_todo_task::{config::AppConfig, storage::TaskStorage, task_service::TaskService, mcp_handler::TaskMcpHandler};
//! use rmcp::service::ServiceExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AppConfig::from_env();
//!     let storage = TaskStorage::new(config.tasks_file_path);
//!     let task_service = TaskService::new(storage);
//!     let handler = TaskMcpHandler::new(task_service);
//!     
//!     let transport = (tokio::io::stdin(), tokio::io::stdout());
//!     let _running_server = handler.serve(transport).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod mcp_handler;
pub mod models;
pub mod storage;
pub mod task_service;

// Re-export commonly used types
pub use config::AppConfig;
pub use mcp_handler::TaskMcpHandler;
pub use models::{Priority, Task, TaskCollection, TaskStatus};
pub use storage::TaskStorage;
pub use task_service::{TaskService, TaskStatistics};
