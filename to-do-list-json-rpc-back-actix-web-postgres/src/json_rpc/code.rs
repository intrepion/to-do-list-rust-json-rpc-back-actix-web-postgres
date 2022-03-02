#[cfg(test)]
mod tests {
    mod test_code {
        mod default_should {
            #[test]
            fn return_new_code() {
                let expected = r#"{0}"#;
                let actual = serde_json::to_string(&Code::default()).unwrap();
            }
        }
    }
}
