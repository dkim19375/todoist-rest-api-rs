//! Structures representing objects in the Todoist Sections API (<https://developer.todoist.com/rest/v2/?shell#sections>)

use serde::Deserialize;

use crate::todoist_config::TodoistConfig;
use crate::RequestError;

/// A Todoist section (<https://developer.todoist.com/rest/v2/?shell#sections>)
#[derive(Debug, Deserialize, Clone)]
pub struct Section {
    /// The section ID
    pub id: String,
    /// The ID of the [Project](crate::model::project::Project) that the section belongs to
    pub project_id: String,
    /// The section position among other sections from the same [Project](crate::model::project::Project)
    pub order: u32,
    /// The section name
    pub name: String,
}

impl Section {
    /// Creates a new copy of this [Section] with a new ID from the Todoist API
    ///
    /// This method is a shortcut for [`todoist_rest_api::sections::create_new_section(config, section.project_id, section.name, Some(section.order))`](crate::sections::create_new_section)
    pub async fn create_new_copy(self, config: &TodoistConfig) -> Result<Section, RequestError> {
        crate::sections::create_new_section(config, self.project_id, self.name, Some(self.order))
            .await
    }

    /// Retrieves the updated section from the Todoist API using this section's ID.
    ///
    /// If the section no longer exists, then a [RequestError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::sections::get_section(config, section.id)`](crate::sections::get_section)
    pub async fn retrieve_updated(self, config: &TodoistConfig) -> Result<Section, RequestError> {
        crate::sections::get_section(config, self.id).await
    }

    /// Updates the section using the Todoist API.
    ///
    /// If the section no longer exists, then a [RequestError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::sections::update_section(config, section.id, section.name)`](crate::sections::update_section)
    pub async fn update(self, config: &TodoistConfig) -> Result<Section, RequestError> {
        crate::sections::update_section(config, self.id, self.name).await
    }

    /// Deletes the section using the Todoist API.
    ///
    /// If the section no longer exists, then a [RequestError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::sections::delete_section(config, section.id)`](crate::sections::delete_section)
    pub async fn delete(self, config: &TodoistConfig) -> Result<(), RequestError> {
        crate::sections::delete_section(config, self.id).await
    }
}
