use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

use crate::models::TaskCollection;

/// Task storage handler responsible for persisting and loading tasks
#[derive(Debug, Clone)]
pub struct TaskStorage {
    file_path: PathBuf,
}

impl TaskStorage {
    /// Create a new task storage instance with the specified file path
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Load tasks from the JSON file
    /// If the file doesn't exist, returns an empty task collection
    pub async fn load_tasks(&self) -> Result<TaskCollection> {
        if !self.file_path.exists() {
            // Return empty collection if file doesn't exist
            return Ok(TaskCollection::new());
        }

        let content = fs::read_to_string(&self.file_path).await?;
        let tasks: TaskCollection = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    /// Save tasks to the JSON file
    pub async fn save_tasks(&self, tasks: &TaskCollection) -> Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.file_path, content).await?;
        Ok(())
    }

    /// Get the file path being used for storage
    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Check if the storage file exists
    pub fn file_exists(&self) -> bool {
        self.file_path.exists()
    }
}
