#[cfg(test)]
mod tests {
    mod test_error {
        mod default_should {
            use super::super::super::ErrorDetail;
            use serde_json;

            #[test]
            fn return_new_error() {
                let expected = r#"{"code":0,"message":"This is the default error message."}"#;
                let actual = serde_json::to_string(&ErrorDetail::default()).unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}

use super::code::Code;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorDetail {
    code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    message: String,
}

impl ErrorDetail {
    pub fn default() -> Self {
        ErrorDetail {
            code: Code::default(),
            data: None,
            message: "This is the default error message.".to_owned(),
        }
    }
}
