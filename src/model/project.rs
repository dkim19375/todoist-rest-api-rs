use serde::{Deserialize, Serialize};

use crate::model::color::Color;

/// A Todoist Project
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProjectViewStyle {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "board")]
    Board,
}