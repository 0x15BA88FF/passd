use std::{io, fs, path};
use crate::utils::{config::CONFIG, path_lib};
use crate::server::{
    request::Parameter, response::ApiError, response::ApiResponse, response::ErrorType,
};

pub fn mkdir(path: &path::PathBuf) -> Result<(), io::Error> {
    if path.exists() {
        if path.is_dir() { Ok(()) }
        else {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Path exists but is not a directory: {}", path.display()),
            ))
        }
    }
    else { fs::create_dir_all(path)?; Ok(()) }
}

pub fn handle(parameters: Option<Vec<Parameter>>) -> ApiResponse {
    let store_directory = path_lib::expand_path(&CONFIG.store_directory);
    let mut target_paths: Vec<path::PathBuf> = vec![];

    let mut paths: Vec<String> = parameters
        .unwrap_or_default()
        .into_iter()
        .filter(|param| param.key == "path")
        .map(|param| param.value)
        .collect();
    paths.dedup();
    
    for path in &paths {
        let mut full_path = store_directory.clone();
        full_path.push(path);
        target_paths.push(full_path);
    }

    for path in &target_paths {
        match mkdir(&path) {
            Ok(_) => continue,
            Err(e) => {
                return ApiResponse {
                    data: None,
                    error: None,
                    status: 200,
                    success: true,
                    message: "Successfully created directory".to_string(),
                }
            }
        }
    }

    ApiResponse {
        data: None,
        error: None,
        status: 200,
        success: true,
        message: "Successfully created directory".to_string(),
    }
}
