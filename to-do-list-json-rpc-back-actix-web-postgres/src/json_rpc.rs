#[cfg(test)]
mod tests {
    mod json_rpc_error {
        mod default_should {
            #[test]
            fn return_new_error() {
                let _expected =
                    r#"{"json_rpc":{"code":1,"message":"This is the default error message."}}"#;
                let actual = JsonRpcError::default().to_string();
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonRpcError {
    code: String,
    data: Option<serde_json::Value>,
    message: String,
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub id: Option<String>,
    pub json_rpc: String,
    pub method: ToDoListMethod,
    pub params: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub error: Option<JsonRpcError>,
    pub id: Option<String>,
    pub json_rpc: String,
    pub result: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub enum ToDoListMethod {
    HealthCheck,
}
