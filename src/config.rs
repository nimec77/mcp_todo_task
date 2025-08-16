use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Path to the tasks JSON file
    pub tasks_file_path: PathBuf,
}

impl AppConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let tasks_file_path = std::env::var("TASKS_FILE")
            .unwrap_or_else(|_| "./data/tasks.json".to_string())
            .into();

        Self { tasks_file_path }
    }

    /// Create configuration with custom file path
    pub fn with_file_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            tasks_file_path: path.into(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AppConfig::from_env();
        // Should use default path when env var is not set
        assert!(config.tasks_file_path.to_string_lossy().contains("tasks.json"));
    }

    #[test]
    fn test_custom_config() {
        let custom_path = "/custom/path/tasks.json";
        unsafe {
            env::set_var("TASKS_FILE", custom_path);
        }
        
        let config = AppConfig::from_env();
        assert_eq!(config.tasks_file_path.to_string_lossy(), custom_path);
        
        unsafe {
            env::remove_var("TASKS_FILE");
        }
    }

    #[test]
    fn test_with_file_path() {
        let custom_path = "/another/path/tasks.json";
        let config = AppConfig::with_file_path(custom_path);
        assert_eq!(config.tasks_file_path.to_string_lossy(), custom_path);
    }
}
