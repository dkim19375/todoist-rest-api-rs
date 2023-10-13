//! Structures representing objects in the Todoist Comments API (<https://developer.todoist.com/rest/v2/?shell#comments>)

use serde::{Deserialize, Serialize};

/// A Todoist comment (<https://developer.todoist.com/rest/v2/?shell#comments>)
#[derive(Debug, Deserialize, Clone)]
pub struct Comment {
    /// The comment ID
    pub id: String,
    /// The comment's [task](crate::model::task::Task) ID
    /// (will be [None] if the comment belongs to a project)
    pub task_id: Option<String>,
    /// The comment's [project](crate::model::project::Project) ID
    /// (will be [None] if the comment belongs to a task)
    pub project_id: Option<String>,
    /// The comment's content which may contain [markdown-formatted text and hyperlinks](https://todoist.com/help/articles/205195102)
    pub content: String,
    /// The attachment file (will be [None] if there is no attachment)
    pub attachment: Option<CommentAttachment>,
}

// TODO - figure out what attributes go here
/// A structure that represents an attachment for a Todoist comment.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommentAttachment {
    /// The name of the file
    pub file_name: String,
    /// The attachment's file type (e.g. `application/pdf`)
    pub file_type: String,
    /// The attachment's URL
    pub file_url: String,
    /// The attachment's resource type (e.g. `file`)
    pub resource_type: String,
}
