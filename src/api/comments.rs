//! Todoist Comments API (<https://developer.todoist.com/rest/v2/?shell#comments>)

use crate::internal::request::models::{CreateNewCommentArgs, UpdateCommentArgs};
use crate::internal::request::paths::create_path;
use crate::internal::request::{
    paths, send_todoist_delete_request, send_todoist_get_request, send_todoist_post_request,
    TodoistAPIError,
};
use crate::model::comment::{Comment, CommentAttachment};
use crate::todoist_config::TodoistConfig;

/// Get all [comments](Comment) from a [task](crate::model::task::Task)
/// or [project](crate::model::project::Project)
pub async fn get_all_comments(
    config: &TodoistConfig,
    task_or_project_id: &TaskOrProjectID,
) -> Result<Vec<Comment>, TodoistAPIError> {
    send_todoist_get_request(
        config,
        format!(
            "{}?{}={}",
            paths::COMMENTS,
            task_or_project_id.get_key(),
            task_or_project_id.get_id()
        ),
    )
    .await
}

/// Creates a new [comment](Comment) on a project or task
pub async fn create_new_comment(
    config: &TodoistConfig,
    task_or_project_id: &TaskOrProjectID,
    content: String,
    attachment: Option<CommentAttachment>,
) -> Result<Comment, TodoistAPIError> {
    send_todoist_post_request(
        config,
        paths::COMMENTS.to_string(),
        Some(&CreateNewCommentArgs {
            task_id: match task_or_project_id {
                TaskOrProjectID::Task(id) => Some(id.clone()),
                TaskOrProjectID::Project(_) => None,
            },
            project_id: match task_or_project_id {
                TaskOrProjectID::Task(_) => None,
                TaskOrProjectID::Project(id) => Some(id.clone()),
            },
            content,
            attachment,
        }),
        true,
    )
    .await
}

/// Gets a single [comment](Comment)
pub async fn get_comment(
    config: &TodoistConfig,
    comment_id: String,
) -> Result<Comment, TodoistAPIError> {
    send_todoist_get_request(config, get_comment_path(comment_id)).await
}

/// Updates a [comment](Comment)
pub async fn update_comment(
    config: &TodoistConfig,
    comment_id: String,
    content: String,
) -> Result<Comment, TodoistAPIError> {
    send_todoist_post_request(
        config,
        get_comment_path(comment_id),
        Some(&UpdateCommentArgs { content }),
        true,
    )
    .await
}

/// Deletes a [comment](Comment)
pub async fn delete_comment(
    config: &TodoistConfig,
    comment_id: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_delete_request(config, get_comment_path(comment_id)).await
}

fn get_comment_path(comment_id: String) -> String {
    create_path(&[paths::COMMENTS, &comment_id])
}

/// An enum to represent either a [task](crate::model::task::Task) or a [project](crate::model::project::Project) ID
#[derive(Debug)]
pub enum TaskOrProjectID {
    /// A [task](crate::model::task::Task) ID
    Task(String),
    /// A [project](crate::model::project::Project) ID
    Project(String),
}

impl TaskOrProjectID {
    fn get_id(&self) -> &String {
        match self {
            TaskOrProjectID::Task(id) => id,
            TaskOrProjectID::Project(id) => id,
        }
    }

    fn get_key(&self) -> String {
        match self {
            TaskOrProjectID::Task(_) => "task_id",
            TaskOrProjectID::Project(_) => "project_id",
        }
        .to_string()
    }
}
