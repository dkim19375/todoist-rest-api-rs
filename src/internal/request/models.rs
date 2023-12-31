use serde::Serialize;

use crate::model::color::Color;
use crate::model::comment::CommentAttachment;
use crate::model::project::ProjectViewStyle;
use crate::model::task::TaskDurationUnit;

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewCommentArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<CommentAttachment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCommentArgs {
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewPersonalLabelArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePersonalLabelArgs {
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameSharedLabelsArgs {
    pub name: String,
    pub new_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveSharedLabelsArgs {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewProjectArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<ProjectViewStyle>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProjectArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<ProjectViewStyle>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewSectionArgs {
    pub name: String,
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSectionArgs {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNewTaskArgs {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<u32>,
    pub labels: Option<Vec<String>>,
    pub priority: Option<u8>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub assignee_id: Option<String>,
    pub duration: Option<u64>,
    pub duration_unit: Option<TaskDurationUnit>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTaskArgs {
    pub content: Option<String>,
    pub description: Option<String>,
    pub labels: Option<Vec<String>>,
    pub priority: Option<u8>,
    pub due_string: Option<String>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub assignee_id: Option<String>,
    pub duration: Option<u64>,
    pub duration_unit: Option<TaskDurationUnit>,
}
