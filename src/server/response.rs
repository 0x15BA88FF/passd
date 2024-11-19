use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ErrorType {
    NotFound,
    Unauthorized,
    InternalError,
    InvalidRequest,
    ValidationError,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub status: u16,
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
    pub error: Option<ApiError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub r#type: Option<ErrorType>,
    pub message: String,
}
