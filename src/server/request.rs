use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRequest {
    pub command: String,
    pub parameters: Option<Vec<Parameter>>,
    pub metadata: Option<RequestMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMetadata {
    pub request_id: String,
    pub timestamp: String,
}
