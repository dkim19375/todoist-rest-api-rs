//! Structures and enums representing objects in the Todoist Projects API (<https://developer.todoist.com/rest/v2/?shell#projects>)

use serde::{Deserialize, Serialize};

use crate::comments::TaskOrProjectID;
use crate::internal::request::TodoistAPIError;
use crate::model::color::Color;
use crate::model::comment::{Comment, CommentAttachment};
use crate::todoist_config::TodoistConfig;

/// A Todoist project (<https://developer.todoist.com/rest/v2/?shell#projects>)
#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    /// The project ID
    pub id: String,
    /// The project name
    pub name: String,
    /// The [color](Color) of the project icon
    pub color: Color,
    /// The ID of the parent project ([None] for top-level projects)
    pub parent_id: Option<String>,
    /// The project position under the same parent (read-only, will be `0` for inbox and team inbox projects)
    pub order: u32,
    /// The number of project comments
    pub comment_count: u32,
    /// Whether the project is shared (read-only)
    pub is_shared: bool,
    /// Whether the project is a favorite
    pub is_favorite: bool,
    /// Whether the project is the user's `Inbox` (read-only)
    pub is_inbox_project: bool,
    /// Whether the project is the `Team Inbox` (read-only)
    pub is_team_inbox: bool,
    /// The way the project is displayed within the Todoist clients
    pub view_style: ProjectViewStyle,
    /// The URL to access this project in the Todoist web or mobile applications
    pub url: String,
}

impl Project {
    /// Creates a new copy of this [Project] with a new ID from the Todoist API
    ///
    /// This method is a shortcut for [`todoist_rest_api::projects::create_new_project(config, project.name, project.parent_id, Some(project.color), Some(project.is_favorite), Some(project.view_style))`](crate::projects::create_new_project)
    pub async fn create_new_copy(self, config: &TodoistConfig) -> Result<Project, TodoistAPIError> {
        crate::projects::create_new_project(
            config,
            self.name,
            self.parent_id,
            Some(self.color),
            Some(self.is_favorite),
            Some(self.view_style),
        )
        .await
    }

    /// Retrieves the updated project from the Todoist API using this project's ID.
    ///
    /// If the project no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::projects::get_project(config, project.id)`](crate::projects::get_project)
    pub async fn retrieve_updated(
        self,
        config: &TodoistConfig,
    ) -> Result<Project, TodoistAPIError> {
        crate::projects::get_project(config, self.id).await
    }

    /// Updates and overwrites the project in Todoist with this [Project]
    ///
    /// If the project no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::projects::update_project(config, project.id, Some(project.name), Some(project.color), Some(project.is_favorite), Some(project.view_style))`](crate::projects::update_project)
    pub async fn update(self, config: &TodoistConfig) -> Result<Project, TodoistAPIError> {
        crate::projects::update_project(
            config,
            self.id,
            Some(self.name),
            Some(self.color),
            Some(self.is_favorite),
            Some(self.view_style),
        )
        .await
    }

    // Todo check if error is actually returned
    /// Deletes this project from Todoist
    ///
    /// If the project no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::projects::delete_project(config, project.id)`](crate::projects::delete_project)
    pub async fn delete(self, config: &TodoistConfig) -> Result<(), TodoistAPIError> {
        crate::projects::delete_project(config, self.id).await
    }

    /// Get all of the [comments](Comment) for this project
    ///
    /// If the project no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::comments::get_all_comments(config, &TaskOrProjectID::Project(project.id))`](crate::comments::get_all_comments)
    pub async fn get_all_comments(
        self,
        config: &TodoistConfig,
    ) -> Result<Vec<Comment>, TodoistAPIError> {
        crate::comments::get_all_comments(config, &TaskOrProjectID::Project(self.id)).await
    }

    /// Creates a new [comment](Comment) for this project
    ///
    /// If the project no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::comments::create_new_comment(config, &TaskOrProjectID::Project(project.id), content, attachment)`](crate::comments::create_new_comment)
    pub async fn create_new_comment(
        self,
        config: &TodoistConfig,
        content: String,
        attachment: Option<CommentAttachment>,
    ) -> Result<Comment, TodoistAPIError> {
        crate::comments::create_new_comment(
            config,
            &TaskOrProjectID::Project(self.id),
            content,
            attachment,
        )
        .await
    }
}

/// The way that the user views the project in the Todoist clients
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectViewStyle {
    /// The project is displayed as a list
    #[serde(rename = "list")]
    List,
    /// The project is displayed as a board
    #[serde(rename = "board")]
    Board,
}
