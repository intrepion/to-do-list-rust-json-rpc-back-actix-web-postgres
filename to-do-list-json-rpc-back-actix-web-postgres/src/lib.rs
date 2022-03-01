#[cfg(test)]
mod tests {
    mod health_check_should {
        use super::super::{api, JsonRpc2Point0Request, ToDoListMethod};
        use actix_web::{middleware, test, App};

        #[actix_web::test]
        async fn return_ok() {
            let app =
                test::init_service(App::new().wrap(middleware::Logger::default()).service(api))
                    .await;
            let req = test::TestRequest::post()
                .uri("/api")
                .set_json(JsonRpc2Point0Request {
                    id: Some("1".to_owned()),
                    method: ToDoListMethod::HealthCheck,
                    params: None,
                })
                .to_request();
            let _resp: JsonRpc2Point0Response = test::call_and_read_body_json(&mut app, req).await;
        }
    }
}

use actix_web::{post, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonRpc2Point0Request {
    pub id: Option<String>,
    pub method: ToDoListMethod,
    pub params: Option<String>,
}

#[derive(Serialize)]
pub enum ToDoListMethod {
    HealthCheck,
}

#[post("/api")]
pub async fn api() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("{}"))
}
