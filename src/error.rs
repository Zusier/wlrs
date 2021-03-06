use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum WlrsError {
    WLRS_ERROR_NOT_LOGGED_IN,
    WLRS_ERROR_NOT_FOUND,
    WLRS_ERROR_WORKOUT_NOT_FOUND,
    WLRS_ERROR_INVALID_TYPE,
    WLRS_ERROR_USERNAME_EXISTS,
    Custom { message: String },
}

impl fmt::Display for WlrsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WLRS_ERROR_NOT_FOUND => f.write_str("Not found!"),
            Self::WLRS_ERROR_WORKOUT_NOT_FOUND => f.write_str("Workout not found!"),
            Self::WLRS_ERROR_NOT_LOGGED_IN => f.write_str("Not logged in!"),
            Self::WLRS_ERROR_INVALID_TYPE => f.write_str("Invalid type found!"),
            Self::WLRS_ERROR_USERNAME_EXISTS => f.write_str("Username already exists!"),
            Self::Custom {
                message,
            } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for WlrsError {}

impl From<WlrsError> for serde_json::Value {
    fn from(error: WlrsError) -> serde_json::Value {
        serde_json::json!({
            "error": "Backend Error",
            "message": error.to_string(),
        })
    }
}
