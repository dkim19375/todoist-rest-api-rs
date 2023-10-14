/*!
Todoist Labels API (<https://developer.todoist.com/rest/v2/?shell#labels>)

There are two different types of labels in Todoist - **personal labels** and **shared labels**.

For information about the differences between these two types, see
<https://developer.todoist.com/rest/v2/?shell#labels> and <https://todoist.com/help/articles/introduction-to-labels-dSo2eE#shared>.
*/

use crate::internal::request::models::{
    CreateNewPersonalLabelArgs, RemoveSharedLabelsArgs, RenameSharedLabelsArgs,
    UpdatePersonalLabelArgs,
};
use crate::internal::request::paths::create_path;
use crate::internal::request::{
    paths, send_todoist_delete_request, send_todoist_get_request, send_todoist_post_request,
    TodoistAPIError,
};
use crate::model::color::Color;
use crate::model::label::PersonalLabel;
use crate::todoist_config::TodoistConfig;

/// Get all personal user [labels](PersonalLabel)
pub async fn get_all_personal_labels(
    config: &TodoistConfig,
) -> Result<Vec<PersonalLabel>, TodoistAPIError> {
    send_todoist_get_request(config, paths::LABELS.into()).await
}

/// Create a new personal [PersonalLabel]
///
/// # Arguments
/// * `name` - The name of the label
/// * `order` - The number used by clients to sort the list of labels (optional)
/// * `color` - The [color](Color) of the label icon (optional)
/// * `is_favorite` - Whether the label is a favorite (optional)
pub async fn create_new_personal_label(
    config: &TodoistConfig,
    name: String,
    order: Option<u32>,
    color: Option<Color>,
    is_favorite: Option<bool>,
) -> Result<PersonalLabel, TodoistAPIError> {
    send_todoist_post_request(
        config,
        paths::LABELS.into(),
        Some(&CreateNewPersonalLabelArgs {
            name,
            order,
            color,
            is_favorite,
        }),
        true,
    )
    .await
}

/// Get a personal [PersonalLabel] by ID
pub async fn get_personal_label(
    config: &TodoistConfig,
    label_id: String,
) -> Result<PersonalLabel, TodoistAPIError> {
    send_todoist_get_request(config, get_label_path(label_id)).await
}

/// Update a personal [PersonalLabel]
pub async fn update_personal_label(
    config: &TodoistConfig,
    label_id: String,
    name: Option<String>,
    order: Option<u32>,
    color: Option<Color>,
    is_favorite: Option<bool>,
) -> Result<PersonalLabel, TodoistAPIError> {
    send_todoist_post_request(
        config,
        get_label_path(label_id),
        Some(&UpdatePersonalLabelArgs {
            name,
            order,
            color,
            is_favorite,
        }),
        true,
    )
    .await
}

/// Delete a personal [PersonalLabel]
///
/// Deleting a [PersonalLabel] will remove all instances of it from tasks
pub async fn delete_personal_label(
    config: &TodoistConfig,
    label_id: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_delete_request(config, get_label_path(label_id)).await
}

/// Get the names of all shared labels currently assigned to tasks.
///
/// By default, the names of a user's **personal** labels will also be included.
///
/// These can be excluded by setting the `omit_personal` parameter to `true`.
pub async fn get_all_shared_labels(
    config: &TodoistConfig,
    omit_personal: Option<bool>,
) -> Result<Vec<String>, TodoistAPIError> {
    send_todoist_get_request(
        config,
        format!(
            "{}{}",
            paths::LABELS_SHARED,
            match omit_personal {
                Some(omit_personal) => format!("?omit_personal={}", omit_personal),
                None => "".to_string(),
            },
        ),
    )
    .await
}

/// Rename all instances of a shared label
///
/// # Arguments
/// * `name` - The name of the existing label to rename
/// * `new_name` - The new name for the label
pub async fn rename_shared_labels(
    config: &TodoistConfig,
    name: String,
    new_name: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_post_request(
        config,
        paths::LABELS_SHARED_RENAME.into(),
        Some(&RenameSharedLabelsArgs { name, new_name }),
        true,
    )
    .await
}

/// Remove all instances of a shared label.
///
/// If no instances of the label name are found, the request will still be considered successful.
pub async fn remove_shared_labels(
    config: &TodoistConfig,
    name: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_post_request(
        config,
        paths::LABELS_SHARED_REMOVE.into(),
        Some(&RemoveSharedLabelsArgs { name }),
        true,
    )
    .await
}

fn get_label_path(label_id: String) -> String {
    create_path(&[paths::LABELS, &label_id])
}
