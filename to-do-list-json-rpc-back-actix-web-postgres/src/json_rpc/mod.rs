pub mod code;
pub mod error;
pub mod error_detail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub id: Option<String>,
    pub json_rpc: String,
    pub method: ToDoListMethod,
    pub params: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub error: Option<error::Error>,
    pub id: Option<String>,
    pub json_rpc: String,
    pub result: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub enum ToDoListMethod {
    HealthCheck,
}
