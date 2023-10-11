use crate::todoist_config::{TodoistConfig, TodoistConfigCreationErrors};

pub fn create_config(token: String) -> Result<TodoistConfig, TodoistConfigCreationErrors> {
    TodoistConfig::new(token)
}