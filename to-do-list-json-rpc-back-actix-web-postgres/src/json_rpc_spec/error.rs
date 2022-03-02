#[cfg(test)]
mod tests {
    mod test_error {
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

use super::error_detail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Error {
    error: error_detail::ErrorDetail,
    id: Option<String>,
    json_rpc: String,
}

impl Error {
    pub fn default() -> Self {
        Error {
            error: error_detail::ErrorDetail::default(),
            id: Some("1".to_owned()),
            json_rpc: "2.0".to_owned(),
        }
    }
}
