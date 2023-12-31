//! Structures representing objects in the Todoist Labels API (<https://developer.todoist.com/rest/v2/?shell#labels>)

use serde::Deserialize;

use crate::internal::request::TodoistAPIError;
use crate::model::color::Color;
use crate::todoist_config::TodoistConfig;

/// A Todoist personal label (<https://developer.todoist.com/rest/v2/?shell#labels>)
#[derive(Debug, Deserialize, Clone)]
pub struct PersonalLabel {
    /// The label ID
    pub id: String,
    /// The label name
    pub name: String,
    /// The [color](Color) of the label icon
    pub color: Color,
    /// The number used by clients to sort the list of labels
    pub order: u32,
    /// Whether the label is a favorite
    pub is_favorite: bool,
}

impl PersonalLabel {
    /// Creates a new copy of this [PersonalLabel] with a new ID from the Todoist API
    ///
    /// This method is a shortcut for
    /// [`todoist_rest_api::labels::create_new_personal_label(config, label.name, Some(label.order), Some(label.color), Some(label.is_favorite))`](crate::labels::create_new_personal_label)
    pub async fn create_new_copy(
        self,
        config: &TodoistConfig,
    ) -> Result<PersonalLabel, TodoistAPIError> {
        crate::labels::create_new_personal_label(
            config,
            self.name,
            Some(self.order),
            Some(self.color),
            Some(self.is_favorite),
        )
        .await
    }

    /// Retrieves the updated label from the Todoist API using this label's ID.
    ///
    /// If the label no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for [`todoist_rest_api::labels::get_personal_label(config, label.id)`](crate::labels::get_personal_label)
    pub async fn retrieve_updated(
        self,
        config: &TodoistConfig,
    ) -> Result<PersonalLabel, TodoistAPIError> {
        crate::labels::get_personal_label(config, self.id).await
    }

    /// Updates and overwrites the label in Todoist with this [PersonalLabel]
    ///
    /// If the label no longer exists, then a [TodoistAPIError] will be returned.
    ///
    /// This method is a shortcut for
    /// [`todoist_rest_api::labels::update_personal_label(config, label.id, Some(label.name), Some(label.order), Some(label.color), Some(label.is_favorite))`](crate::labels::update_personal_label)
    pub async fn update(self, config: &TodoistConfig) -> Result<PersonalLabel, TodoistAPIError> {
        crate::labels::update_personal_label(
            config,
            self.id,
            Some(self.name),
            Some(self.order),
            Some(self.color),
            Some(self.is_favorite),
        )
        .await
    }

    /// Deletes this [PersonalLabel]
    ///
    /// This method is a shortcut for [`todoist_rest_api::labels::delete_personal_label(config, label.id)`](crate::labels::delete_personal_label)
    pub async fn delete(self, config: &TodoistConfig) -> Result<(), TodoistAPIError> {
        crate::labels::delete_personal_label(config, self.id).await
    }
}
