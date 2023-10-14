//! Todoist Sections API (<https://developer.todoist.com/rest/v2/?shell#sections>)

use crate::internal::request::models::{CreateNewSectionArgs, UpdateSectionArgs};
use crate::internal::request::paths::create_path;
use crate::internal::request::{
    paths, send_todoist_delete_request, send_todoist_get_request, send_todoist_post_request,
};
use crate::model::section::Section;
use crate::todoist_config::TodoistConfig;
use crate::TodoistAPIError;

/// Get all [sections](Section), optionally filtering the returned sections with a project ID
pub async fn get_all_sections(
    config: &TodoistConfig,
    project_id: Option<String>,
) -> Result<Vec<Section>, TodoistAPIError> {
    send_todoist_get_request(
        config,
        match project_id {
            Some(id) => create_path(&[paths::SECTIONS, paths::PARAM_PROJECT_ID, &id]),
            None => paths::SECTIONS.to_string(),
        },
    )
    .await
}

/// Create a new [Section]
pub async fn create_new_section(
    config: &TodoistConfig,
    project_id: String,
    name: String,
    order: Option<u32>,
) -> Result<Section, TodoistAPIError> {
    send_todoist_post_request(
        config,
        paths::SECTIONS.to_string(),
        Some(&CreateNewSectionArgs {
            project_id,
            name,
            order,
        }),
        false,
    )
    .await
}

/// Get a single [Section]
pub async fn get_section(
    config: &TodoistConfig,
    section_id: String,
) -> Result<Section, TodoistAPIError> {
    send_todoist_get_request(config, get_section_path(section_id)).await
}

/// Update a [Section]
pub async fn update_section(
    config: &TodoistConfig,
    section_id: String,
    name: String,
) -> Result<Section, TodoistAPIError> {
    send_todoist_post_request(
        config,
        get_section_path(section_id),
        Some(&UpdateSectionArgs { name }),
        false,
    )
    .await
}

/// Delete a [Section]
pub async fn delete_section(
    config: &TodoistConfig,
    section_id: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_delete_request(config, get_section_path(section_id)).await
}

fn get_section_path(section_id: String) -> String {
    create_path(&[paths::SECTIONS, &section_id])
}
