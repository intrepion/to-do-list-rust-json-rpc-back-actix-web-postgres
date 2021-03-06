#[cfg(test)]
mod tests {
    mod test_code {
        mod default_should {
            use super::super::super::Code;

            #[test]
            fn return_new_code() {
                let expected = r#"0"#;
                let actual = serde_json::to_string(&Code::default()).unwrap();

                assert_eq!(actual, expected);
            }
        }

        mod new_should {
            use super::super::super::Code;

            #[test]
            fn return_new_code_given_valid_code() {
                let expected = r#"1"#;
                let actual = serde_json::to_string(&Code::new(1)).unwrap();

                assert_eq!(actual, expected);
            }
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Code(i64);

impl Code {
    pub fn default() -> Self {
        Self::new(0)
    }

    pub fn new(code: i64) -> Self {
        Code(code)
    }
}
