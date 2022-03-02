#[cfg(test)]
mod tests {
    mod health_check_should {
        use super::super::api;
        use crate::json_rpc_spec;
        use actix_web::{middleware, test, App};

        #[actix_web::test]
        async fn return_ok() {
            let mut app =
                test::init_service(App::new().wrap(middleware::Logger::default()).service(api))
                    .await;
            let req = test::TestRequest::post()
                .uri("/api")
                .set_json(json_rpc_spec::JsonRpcRequest {
                    id: Some("1".to_owned()),
                    json_rpc: "2.0".to_owned(),
                    method: json_rpc_spec::ToDoListMethod::HealthCheck,
                    params: None,
                })
                .to_request();
            let _resp: json_rpc_spec::JsonRpcResponse =
                test::call_and_read_body_json(&mut app, req).await;
        }
    }
}

use crate::json_rpc_spec;
use actix_web::{post, web, Error, HttpResponse};

#[post("/api")]
pub async fn api(req: web::Json<json_rpc_spec::JsonRpcRequest>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(json_rpc_spec::JsonRpcResponse {
        error: None,
        id: req.id.clone(),
        json_rpc: "2.0".to_owned(),
        result: None,
    }))
}
