use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

/// Error class for when functions return a error in json format
/// Credit for blake-mealey for his implementation of the ApiError struct.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authorization failed. Check your .ROBLOSECURITY cookie.")]
    AuthorizationFailed,

    #[error("Roblox error ({status_code}) : {reason}")]
    Roblox {
        status_code: StatusCode,
        reason: String,
    },

    #[error("Failed to parse JSON response: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("Request Error")]
    RequestError { reason: String },
}

impl From<ApiError> for String {
    fn from(e: ApiError) -> Self {
        e.to_string()
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::RequestError {
            reason: value.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RobloxApiErrorResponse {
    // Most errors include a `message` property
    #[serde(alias = "Message")]
    pub message: Option<String>,

    // Error models (500) which have a `title` property instead
    #[serde(alias = "Title")]
    pub title: Option<String>,

    // some error models on older APIs have an errors array
    #[serde(alias = "Errors")]
    pub errors: Option<Vec<RobloxApiErrorResponse>>,

    // Some errors return a `sucess` property which can be used to check for errors
    #[serde(alias = "Success")]
    pub success: Option<bool>,
}

impl RobloxApiErrorResponse {
    pub fn is_empty(&self) -> bool {
        self.message.is_none()
            && self.title.is_none()
            && self.errors.is_none()
            && self.success.is_none()
    }

    pub fn reason(self) -> Option<String> {
        if let Some(message) = self.message {
            return Some(message);
        } else if let Some(title) = self.title {
            return Some(title);
        } else if let Some(errors) = self.errors {
            for error in errors {
                if let Some(message) = error.reason() {
                    return Some(message);
                }
            }
            return None;
        }
        None
    }
}
