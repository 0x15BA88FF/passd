use serde_json::Value;
use std::{path, io, fs};
use serde::{Deserialize, Serialize};
use crate::utils::{config::CONFIG, path_lib};
use crate::server::{
    request::Parameter, response::ApiError, response::ApiResponse, response::ErrorType,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FsItemType { File, Directory }

#[derive(Debug, Serialize, Deserialize)]
pub struct FsItem {
    pub name: String,
    pub path: String,
    pub r#type: FsItemType,
    pub items: Option<Vec<FsItem>>,
}

pub fn get_entities(path: &path::PathBuf) -> Result<FsItem, io::Error> {
    let name = path
        .file_name()
        .map(|os_str| os_str.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unnamed".to_string());

    if path.is_dir() {
        let items = {
            let mut children = Vec::new();
            for entry in fs::read_dir(path)? {
                children.push(get_entities(&entry?.path())?);
            }
            Some(children)
        };
        Ok(FsItem {
            name,
            path: path.display().to_string(),
            r#type: FsItemType::Directory,
            items,
        })
    } else {
        Ok(FsItem {
            name,
            path: path.display().to_string(),
            r#type: FsItemType::File,
            items: None,
        })
    }
}

pub fn list(path: &path::PathBuf) -> Result<Vec<FsItem>, io::Error> {
    let mut children = Vec::new();

    for entity in fs::read_dir(path)? {
        let entity_path = entity?.path();
        match get_entities(&entity_path) {
            Ok(result) => children.push(result),
            Err(e) => return Err(e),
        }
    }
    Ok(children)
}

pub fn handle(parameters: Option<Vec<Parameter>>) -> ApiResponse {
    let store_directory = path_lib::expand_path(&CONFIG.store_directory);
    let mut target_paths: Vec<path::PathBuf> = vec![];

    let mut paths: Vec<String> = parameters
        .unwrap_or_default()
        .into_iter()
        .filter(|param| param.key == "directory")
        .map(|param| param.value)
        .collect();
    paths.dedup();

    for path in &paths {
        let mut full_path = store_directory.clone();
        full_path.push(path);
        target_paths.push(full_path);
    }

    if paths.is_empty() {
        target_paths.push(store_directory.clone());
    }

    let mut content: Vec<FsItem> = vec![];

    for path in target_paths {
        match list(&path) {
            Ok(data) => content.extend(data),
            Err(e) => {
                return ApiResponse {
                    data: None,
                    error: Some(ApiError {
                        r#type: Some(ErrorType::InternalError),
                        message: e.to_string(),
                    }),
                    status: 500,
                    success: false,
                    message: "Failed to list entities".to_string(),
                };
            }
        };
    }

    ApiResponse {
        data: Some(serde_json::to_value(serde_json::json!({
            "content": content
        })).unwrap_or(Value::Null)),
        error: None,
        status: 200,
        success: true,
        message: "Successfully listed entities".to_string(),
    }
}
