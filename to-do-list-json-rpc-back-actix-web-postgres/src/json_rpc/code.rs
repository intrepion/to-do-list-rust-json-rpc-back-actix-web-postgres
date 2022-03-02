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
                let _expected = r#"1"#;
                let _actual = serde_json::to_string(&Code::new(1)).unwrap();
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

    pub fn new(_code: i64) -> Self {
        Code(0)
    }
}
