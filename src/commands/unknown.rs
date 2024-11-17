use serde_json::Value;
use crate::server::{
    response::ApiResponse,
    response::ErrorType,
    response::ApiError,
};

pub fn handle(_parameters: Option<Vec<Value>>) -> ApiResponse {
    ApiResponse {
        data: None,
        status: 400,
        success: false,
        message: "Unknown command".to_string(),
        error: Some(ApiError {
            r#type: Some(ErrorType::NotFound),
            message: "The command provided is not recognized.".to_string()
        }),
    }
}
