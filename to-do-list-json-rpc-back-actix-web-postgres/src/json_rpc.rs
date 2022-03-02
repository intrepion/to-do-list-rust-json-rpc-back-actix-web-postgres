#[cfg(test)]
mod tests {
    mod error {
        mod default_should {
            use super::super::super::Error;
            use serde_json;

            #[test]
            fn return_new_error() {
                let expected = r#"{"error":{"code":0,"message":"This is the default error message."},"id":"1","json_rpc":"2.0"}"#;
                let actual = serde_json::to_string(&Error::default()).unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Error {
    error: ErrorDetail,
    id: Option<String>,
    json_rpc: String,
}

impl Error {
    pub fn default() -> Self {
        Error {
            error: ErrorDetail::default(),
            id: Some("1".to_owned()),
            json_rpc: "2.0".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ErrorDetail {
    code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    message: String,
}

impl ErrorDetail {
    pub fn default() -> Self {
        ErrorDetail {
            code: 0,
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
    pub error: Option<Error>,
    pub id: Option<String>,
    pub json_rpc: String,
    pub result: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub enum ToDoListMethod {
    HealthCheck,
}
