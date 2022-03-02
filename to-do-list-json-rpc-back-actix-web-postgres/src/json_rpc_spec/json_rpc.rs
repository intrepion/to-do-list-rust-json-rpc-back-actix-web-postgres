#[cfg(test)]
mod tests {
    mod test_json_rpc {
        mod default_should {
            use super::super::super::JsonRpc;

            #[test]
            fn return_new_json_rpc() {
                let _expected = r#""2.0""#;
                let _actual = serde_json::to_string(&JsonRpc::default()).unwrap();
            }
        }

        mod new_should {
            use super::super::super::JsonRpc;

            #[test]
            fn return_new_json_rpc_given_valid_json_rpc() {
                let _expected = r#""2.0""#;
                let _actual = serde_json::to_string(&JsonRpc::new()).unwrap();
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonRpc(String);

impl JsonRpc {
    pub fn new() -> Self {
        JsonRpc("2.0".to_owned())
    }
}

impl Default for JsonRpc {
    fn default() -> Self {
        JsonRpc::new()
    }
}
