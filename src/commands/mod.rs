pub mod unknown;

use serde_json::Value;
use crate::server::{response::ApiResponse};

type CommandHandler = fn(Option<Vec<Value>>) -> ApiResponse;

pub fn command_handler(command: &str) -> CommandHandler {
    match command {
        _ => unknown::handle,
    }
}
