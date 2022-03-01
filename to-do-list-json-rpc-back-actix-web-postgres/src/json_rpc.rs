#[cfg(test)]
mod tests {
    mod json_rpc_error {
        mod default_should {
            use super::super::super::JsonRpcError;
            use serde_json;

            #[test]
            fn return_new_error() {
                let expected = r#"{"code":1,"message":"This is the default error message."}"#;
                let actual = serde_json::to_string(&JsonRpcError::default()).unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonRpcError {
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    message: String,
}

impl JsonRpcError {
    pub fn default() -> Self {
        JsonRpcError {
            code: 1,
            data: None,
            message: "This is the default error message.".to_owned(),
        }
    }
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
