use crate::server::{
    request::Parameter, response::ApiError, response::ApiResponse, response::ErrorType,
};

pub fn handle(parameters: Option<Vec<Parameter>>) -> ApiResponse {
    ApiResponse {
        data: None,
        error: None,
        status: 200,
        success: true,
        message: "Successfully removed entity".to_string(),
    }
}
