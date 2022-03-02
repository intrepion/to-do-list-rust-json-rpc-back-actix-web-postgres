#[cfg(test)]
mod tests {
    mod test_code {
        mod default_should {
            use super::super::super::Code;

            #[test]
            fn return_new_code() {
                let _expected = r#"{0}"#;
                let _actual = serde_json::to_string(&Code::default()).unwrap();
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Code(i64);

impl Code {
    pub fn default() -> Self {
        Code(0)
    }
}
