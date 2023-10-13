//! Structures representing objects in the Todoist Sections API (<https://developer.todoist.com/rest/v2/?shell#sections>)

use serde::Deserialize;

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
