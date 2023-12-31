use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU16;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::todoist_config::TodoistConfig;

pub mod models;
pub mod paths;

const TODOIST_API_URL: &str = "https://api.todoist.com/rest/v2";

pub async fn send_todoist_get_request<T: DeserializeOwned>(
    config: &TodoistConfig,
    path: String,
) -> Result<T, TodoistAPIError> {
    send_todoist_request::<(), T>(config, path, None, RequestMethod::Get, false)
        .await
        .map(|res| res.unwrap())
}

pub async fn send_todoist_post_request<Req: Serialize + ?Sized, Res: DeserializeOwned>(
    config: &TodoistConfig,
    path: String,
    data: Option<&Req>,
    include_request_id: bool,
) -> Result<Res, TodoistAPIError> {
    send_todoist_request::<Req, Res>(config, path, data, RequestMethod::Post, include_request_id)
        .await
        .map(|res| res.unwrap())
}

pub async fn send_todoist_delete_request(
    config: &TodoistConfig,
    path: String,
) -> Result<(), TodoistAPIError> {
    send_todoist_request::<(), ()>(config, path, None, RequestMethod::Delete, false)
        .await
        .map(|_| ())
}

async fn send_todoist_request<Req: Serialize + ?Sized, Res: DeserializeOwned>(
    config: &TodoistConfig,
    path: String,
    data: Option<&Req>,
    method: RequestMethod,
    include_request_id: bool,
) -> Result<Option<Res>, TodoistAPIError> {
    if !path.starts_with('/') {
        panic!("Path must start with a '/'! Instead was '{}'", path);
    }
    let client = &config.client;
    let mut builder = client.request(method.into(), format!("{}{}", TODOIST_API_URL, &path));
    if let Some(data) = data {
        builder = builder.json(data);
    }
    if include_request_id {
        builder = builder.header("X-Request-Id", Uuid::new_v4().as_simple().to_string());
    }
    let response = builder.send().await?;
    if response.status().is_client_error() {
        return Err(InvalidRequestError {
            status_code: NonZeroU16::new(response.status().as_u16()).unwrap(),
        }
        .into());
    }
    if response.status().is_server_error() {
        return Err(ServerError {
            status_code: NonZeroU16::new(response.status().as_u16()).unwrap(),
        }
        .into());
    }
    if response.status() == 204 {
        return Ok(None);
    }
    let result: Res = response.json().await?;
    Ok(Some(result))
}

#[derive(Debug)]
enum RequestMethod {
    Get,
    Post,
    Delete,
}

impl From<RequestMethod> for Method {
    fn from(value: RequestMethod) -> Self {
        match value {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
            RequestMethod::Delete => Method::DELETE,
        }
    }
}

/// Errors for when an HTTP request is sent and fails
#[derive(Debug)]
pub enum TodoistAPIError {
    /// Received a 4xx error
    InvalidRequest(InvalidRequestError),
    /// Received a 5xx error
    ServerError(ServerError),
    /// If there was an error while sending the request, a redirect loop was detected,
    /// or the redirect limit was exhausted
    ///
    /// See [reqwest::RequestBuilder::send]
    RequestSendError(reqwest::Error),
    /// If parameters to `todoist_rest_api` are invalid
    APIParametersError(APIParametersError),
    /// If there was an error while parsing a JSON response
    ResponseJSONParseError(serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct InvalidRequestError {
    status_code: NonZeroU16,
}

impl Display for InvalidRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid request - Status code: {}", self.status_code)
    }
}

impl Error for InvalidRequestError {}

#[derive(Debug, Clone)]
pub struct ServerError {
    status_code: NonZeroU16,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server error - Status code: {}", self.status_code)
    }
}

impl Error for ServerError {}

#[derive(Debug, Clone)]
pub struct APIParametersError {
    pub(crate) message: String,
}

impl Display for APIParametersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "API parameters error - {}", self.message)
    }
}

impl Error for APIParametersError {}

impl Error for TodoistAPIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TodoistAPIError::InvalidRequest(ref e) => Some(e),
            TodoistAPIError::ServerError(ref e) => Some(e),
            TodoistAPIError::RequestSendError(ref e) => Some(e),
            TodoistAPIError::ResponseJSONParseError(ref e) => Some(e),
            TodoistAPIError::APIParametersError(ref e) => Some(e),
        }
    }
}

impl Display for TodoistAPIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <dyn Error as Display>::fmt(self.source().unwrap(), f)
    }
}

impl From<InvalidRequestError> for TodoistAPIError {
    fn from(value: InvalidRequestError) -> Self {
        TodoistAPIError::InvalidRequest(value)
    }
}

impl From<ServerError> for TodoistAPIError {
    fn from(value: ServerError) -> Self {
        TodoistAPIError::ServerError(value)
    }
}

impl From<reqwest::Error> for TodoistAPIError {
    fn from(value: reqwest::Error) -> Self {
        TodoistAPIError::RequestSendError(value)
    }
}

impl From<serde_json::Error> for TodoistAPIError {
    fn from(value: serde_json::Error) -> Self {
        TodoistAPIError::ResponseJSONParseError(value)
    }
}

impl From<APIParametersError> for TodoistAPIError {
    fn from(value: APIParametersError) -> Self {
        TodoistAPIError::APIParametersError(value)
    }
}
