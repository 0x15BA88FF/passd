pub mod init;
pub mod list;
pub mod mkdir;
pub mod remove;
pub mod unknown;

use crate::server::{request::Parameter, response::ApiResponse};
use serde_json::Value;

type CommandHandler = fn(Option<Vec<Parameter>>) -> ApiResponse;

pub fn command_handler(command: &str) -> CommandHandler {
    match command {
        "init" => init::handle,
        "list" => list::handle,
        "mkdir" => mkdir::handle,
        "remove" => remove::handle,
        _ => unknown::handle,
    }
}
