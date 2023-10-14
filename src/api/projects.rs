//! Todoist Projects API (<https://developer.todoist.com/rest/v2/?shell#projects>)

use crate::internal::request::models::{CreateNewProjectArgs, UpdateProjectArgs};
use crate::internal::request::paths::create_path;
use crate::internal::request::{
    paths, send_todoist_delete_request, send_todoist_get_request, send_todoist_post_request,
    RequestError,
};
use crate::model::collaborator::Collaborator;
use crate::model::color::Color;
use crate::model::project::{Project, ProjectViewStyle};
use crate::todoist_config::TodoistConfig;

/// Gets all user [projects](Project)
pub async fn get_all_projects(config: &TodoistConfig) -> Result<Vec<Project>, RequestError> {
    send_todoist_get_request(config, paths::PROJECTS.to_string()).await
}

/// Creates a new [project](Project)
///
/// # Arguments
/// * config - The [TodoistConfig] used to use the Todoist API
/// * name - The name of the project
/// * parent_id - The ID of the parent project ([None] for top-level projects)
/// * color - The [color](Color) of the project icon
/// * is_favorite - Whether the project is a favorite
/// * view_style - The way the project is displayed within the Todoist clients
pub async fn create_new_project(
    config: &TodoistConfig,
    name: String,
    parent_id: Option<String>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: Option<ProjectViewStyle>,
) -> Result<Project, RequestError> {
    send_todoist_post_request(
        config,
        paths::PROJECTS.to_string(),
        &CreateNewProjectArgs {
            name,
            parent_id,
            color,
            is_favorite,
            view_style,
        },
        true,
    )
    .await
}

/// Gets a [Project] by its ID
pub async fn get_project(
    config: &TodoistConfig,
    project_id: String,
) -> Result<Project, RequestError> {
    send_todoist_get_request(config, create_path(&[paths::PROJECTS, &project_id])).await
}

/// Updates a project
///
/// # Arguments
/// * config - The [TodoistConfig] used to use the Todoist API
/// * name - The name of the project
/// * color - The [color](Color) of the project icon
/// * is_favorite - Whether the project is a favorite
/// * view_style - The way the project is displayed within the Todoist clients
pub async fn update_project(
    config: &TodoistConfig,
    project_id: String,
    name: Option<String>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: Option<ProjectViewStyle>,
) -> Result<Project, RequestError> {
    send_todoist_post_request(
        config,
        create_path(&[paths::PROJECTS, &project_id]),
        &UpdateProjectArgs {
            name,
            color,
            is_favorite,
            view_style,
        },
        true,
    )
    .await
}

/// Deletes a project
pub async fn delete_project(
    config: &TodoistConfig,
    project_id: String,
) -> Result<(), RequestError> {
    send_todoist_delete_request(config, create_path(&[paths::PROJECTS, &project_id])).await
}

/// Gets all of the collaborators of a shared project
pub async fn get_all_collaborators(
    config: &TodoistConfig,
    project_id: String,
) -> Result<Vec<Collaborator>, RequestError> {
    send_todoist_get_request(
        config,
        create_path(&[paths::PROJECTS, &project_id, paths::COLLABORATORS]),
    )
    .await
}
