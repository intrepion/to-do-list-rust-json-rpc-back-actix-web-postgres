#[cfg(test)]
mod tests {
    mod health_check_should {
        #[actix_web::test]
        async fn return_ok() {
            let app = test::init_service(App::new().wrap(middleware::Logger::default()).service(api)).await();
        }
    }
}
