//! Structures representing objects in the Todoist Labels API (<https://developer.todoist.com/rest/v2/?shell#labels>)

use serde::{Deserialize, Serialize};

use crate::model::color::Color;

/// A Todoist label (<https://developer.todoist.com/rest/v2/?shell#labels>)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Label {
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