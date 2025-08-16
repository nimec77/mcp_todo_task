use anyhow::Result;
use mcp_todo_task::{AppConfig, TaskMcpHandler, TaskService, TaskStorage};
use rmcp::service::ServiceExt;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing to stderr (stdout is used for JSON-RPC)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    info!("Starting Task Manager MCP Server");

    // Load configuration from .env file and environment variables
    let config = AppConfig::load()?;

    // Set up the service layers
    let storage = TaskStorage::new(config.tasks_file_path);
    let task_service = TaskService::new(storage);
    let handler = TaskMcpHandler::new(task_service);

    // Set up transport - using stdio for MCP communication
    let transport = (tokio::io::stdin(), tokio::io::stdout());

    info!("Task Manager MCP Server starting...");

    // Start the server
    let _running_server = handler.serve(transport).await?;

    info!("Task Manager MCP Server is running");

    // Keep the server running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Task Manager MCP Server");

    Ok(())
}
