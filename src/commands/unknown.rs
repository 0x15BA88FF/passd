use crate::server::{
    request::Parameter, response::ApiError, response::ApiResponse, response::ErrorType,
};
use serde_json::Value;

pub fn handle(_parameters: Option<Vec<Parameter>>) -> ApiResponse {
    ApiResponse {
        data: None,
        status: 400,
        success: false,
        message: "Unknown command".to_string(),
        error: Some(ApiError {
            r#type: Some(ErrorType::NotFound),
            message: "The command provided is not recognized.".to_string(),
        }),
    }
}
