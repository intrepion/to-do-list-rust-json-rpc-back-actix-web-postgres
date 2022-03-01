#[cfg(test)]
mod tests {
    mod health_check_should {
        use super::super::api;
        use actix_web::{middleware, test, App};

        #[actix_web::test]
        async fn return_ok() {
            let _app =
                test::init_service(App::new().wrap(middleware::Logger::default()).service(api))
                    .await;
        }
    }
}

use actix_web::{post, Error, HttpResponse};

#[post("/api")]
pub async fn api() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("{}"))
}
