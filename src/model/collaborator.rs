//! Structures representing collaborators in the Todoist API (<https://developer.todoist.com/rest/v2/?shell#get-all-collaborators>)

use serde::Deserialize;

/// A structure representing a collaborator
#[derive(Debug, Deserialize, Clone)]
pub struct Collaborator {
    /// The collaborator's ID (ex: "1234567")
    pub id: String,
    /// The collaborator's name (ex: "Bob")
    pub name: String,
    /// The collaborator's email address (ex: "bob@example.com")
    pub email: String,
}
