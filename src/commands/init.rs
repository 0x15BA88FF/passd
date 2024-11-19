use crate::server::{
    request::Parameter, response::ApiError, response::ApiResponse, response::ErrorType,
};
use crate::utils::{config::CONFIG, path_lib};
use serde_json::Value;
use std::{fs, io};

pub fn init(pgp_keys: Vec<String>) -> Result<(), io::Error> {
    let store_path = path_lib::expand_path(&CONFIG.store_directory);
    let mut gpg_id_path = store_path.clone();
    gpg_id_path.push(".gpg-id");

    fs::create_dir_all(&store_path)?;
    fs::write(&gpg_id_path, pgp_keys.join("\n"))?;
    Ok(())
}

pub fn handle(parameters: Option<Vec<Parameter>>) -> ApiResponse {
    if parameters.is_none() || (parameters.is_some() && parameters.as_ref().unwrap().is_empty()) {
        return ApiResponse {
            data: None,
            status: 400,
            success: false,
            message: "Invalid parameters".to_string(),
            error: Some(ApiError {
                r#type: Some(ErrorType::InvalidRequest),
                message: "Command parameters are required.".to_string(),
            }),
        };
    }

    let mut key_ids: Vec<String> = parameters
        .unwrap_or_default()
        .into_iter()
        .filter(|param| param.key == "key-id")
        .map(|param| param.value).collect::<Vec<String>>();
    key_ids.dedup();

    init(key_ids);

    ApiResponse {
        data: None,
        error: None,
        status: 200,
        success: true,
        message: "Initializing Password Store".to_string(),
    }
}
