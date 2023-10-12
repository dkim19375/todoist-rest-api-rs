use serde::{Deserialize, Serialize};

use crate::model::comment::CommentAttachment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewCommentArgs {
    pub task_id: Option<String>,
    pub project_id: Option<String>,
    pub content: String,
    pub attachment: Option<CommentAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommentArgs {
    pub content: String,
}