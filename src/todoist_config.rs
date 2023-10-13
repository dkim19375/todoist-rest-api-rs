//! A structure to store the Todoist API configuration.

use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

/// A structure to store the Todoist API configuration.
#[derive(Debug, Clone)]
pub struct TodoistConfig {
    /// The Todoist API token (see <https://developer.todoist.com/rest/v2/#authorization>)
    pub token: String,
    #[doc(hidden)]
    pub client: Client,
}

#[doc(hidden)]
impl TodoistConfig {
    pub fn new(token: String) -> Result<TodoistConfig, TodoistConfigCreationErrors> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
            InvalidTokenFormatError { token: token.clone() }
        })?);
        Ok(TodoistConfig {
            token,
            client: Client::builder().default_headers(headers).build()?,
        })
    }
}

/// Creates a [TodoistConfig]
///
/// # Arguments
///
/// * `token` - The Todoist API token to use (see <https://developer.todoist.com/rest/v2/#authorization>)
pub fn create_config(token: String) -> Result<TodoistConfig, TodoistConfigCreationErrors> {
    TodoistConfig::new(token)
}

/// Types of errors that can occur when creating a [TodoistConfig]
#[derive(Debug)]
pub enum TodoistConfigCreationErrors {
    /// An error for when the token format is invalid (such as containing a newline)
    InvalidTokenFormat(InvalidTokenFormatError),
    /// An error for when the `reqwest` [Client] could not be created
    /// From the [reqwest::ClientBuilder::build] documentation:
    /// > This method fails if a TLS backend cannot be initialized,
    /// > or the resolver cannot load the system configuration.
    HttpClientCreationError(reqwest::Error),
}

impl Display for TodoistConfigCreationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <dyn Error as Display>::fmt(self.source().unwrap(), f)
    }
}

impl Error for TodoistConfigCreationErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TodoistConfigCreationErrors::InvalidTokenFormat(ref e) => Some(e),
            TodoistConfigCreationErrors::HttpClientCreationError(ref e) => Some(e),
        }
    }
}

impl From<InvalidTokenFormatError> for TodoistConfigCreationErrors {
    fn from(value: InvalidTokenFormatError) -> Self {
        TodoistConfigCreationErrors::InvalidTokenFormat(value)
    }
}

impl From<reqwest::Error> for TodoistConfigCreationErrors {
    fn from(value: reqwest::Error) -> Self {
        TodoistConfigCreationErrors::HttpClientCreationError(value)
    }
}

/// An error that is thrown when the token is not in the correct format (such as containing a newline)
#[derive(Debug, Clone)]
pub struct InvalidTokenFormatError {
    token: String,
}

impl Display for InvalidTokenFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token format: '{}'", self.token)
    }
}

impl Error for InvalidTokenFormatError {}