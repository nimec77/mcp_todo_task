use serde::{Deserialize, Serialize};

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

impl TaskCollection {
    /// Create a new empty task collection
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            version: "1.0".to_string(),
        }
    }
}

impl Default for TaskCollection {
    fn default() -> Self {
        Self::new()
    }
}
