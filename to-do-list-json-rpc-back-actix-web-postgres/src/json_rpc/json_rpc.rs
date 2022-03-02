#[cfg(test)]
mod tests {
    mod test_json_rpc {
        mod default_should {
            #[test]
            fn return_new_json_rpc() {
                let _expected = r#""2.0""#;
                let _actual = serde_json::to_string(&JsonRpc::default()).unwrap();
            }
        }
    }
}
