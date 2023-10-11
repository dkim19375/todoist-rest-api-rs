use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Debug, Clone)]
pub struct TodoistConfig {
    pub token: String,
    pub client: Client,
}

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

#[derive(Debug)]
pub enum TodoistConfigCreationErrors {
    InvalidTokenFormat(InvalidTokenFormatError),
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