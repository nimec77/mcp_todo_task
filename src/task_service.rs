use anyhow::Result;
use std::collections::HashMap;

use crate::models::{Task, TaskCollection, TaskStatus, Priority};
use crate::storage::TaskStorage;

/// Service for managing task operations and business logic
#[derive(Debug, Clone)]
pub struct TaskService {
    storage: TaskStorage,
}

impl TaskService {
    /// Create a new task service with the given storage
    pub fn new(storage: TaskStorage) -> Self {
        Self { storage }
    }

    /// Load all tasks from storage
    pub async fn load_tasks(&self) -> Result<TaskCollection> {
        self.storage.load_tasks().await
    }

    /// Save tasks to storage
    pub async fn save_tasks(&self, tasks: &TaskCollection) -> Result<()> {
        self.storage.save_tasks(tasks).await
    }

    /// Filter tasks based on criteria
    pub fn filter_tasks(&self, tasks: &[Task], filters: &HashMap<String, String>) -> Vec<Task> {
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

    /// Find a task by ID
    pub async fn find_task_by_id(&self, task_id: &str) -> Result<Option<Task>> {
        let task_collection = self.load_tasks().await?;
        Ok(task_collection.tasks.iter().find(|t| t.id == task_id).cloned())
    }

    /// Get task statistics
    pub async fn get_task_statistics(&self) -> Result<TaskStatistics> {
        let task_collection = self.load_tasks().await?;
        let stats = TaskStatistics::from_tasks(&task_collection.tasks);
        Ok(stats)
    }
}

/// Statistics about tasks
#[derive(Debug, Clone)]
pub struct TaskStatistics {
    pub total_tasks: usize,
    pub status_counts: HashMap<String, usize>,
    pub priority_counts: HashMap<String, usize>,
}

impl TaskStatistics {
    /// Create task statistics from a collection of tasks
    pub fn from_tasks(tasks: &[Task]) -> Self {
        let total_tasks = tasks.len();
        let mut status_counts = HashMap::new();
        let mut priority_counts = HashMap::new();

        for task in tasks {
            *status_counts
                .entry(format!("{:?}", task.status))
                .or_insert(0) += 1;
            *priority_counts
                .entry(format!("{:?}", task.priority))
                .or_insert(0) += 1;
        }

        Self {
            total_tasks,
            status_counts,
            priority_counts,
        }
    }

    /// Format statistics as a human-readable string
    pub fn format_stats(&self) -> String {
        format!(
            "## Task Statistics\n\n**Total Tasks:** {}\n\n### By Status:\n{}\n\n### By Priority:\n{}",
            self.total_tasks,
            self.status_counts
                .iter()
                .map(|(status, count)| format!("- {}: {}", status, count))
                .collect::<Vec<_>>()
                .join("\n"),
            self.priority_counts
                .iter()
                .map(|(priority, count)| format!("- {}: {}", priority, count))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
