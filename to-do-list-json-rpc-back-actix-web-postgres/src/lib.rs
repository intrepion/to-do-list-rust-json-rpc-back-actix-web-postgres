#[cfg(test)]
mod tests {
    mod health_check_should {
        use super::super::{api, JsonRpcRequest, JsonRpcResponse, ToDoListMethod};
        use actix_web::{middleware, test, App};

        #[actix_web::test]
        async fn return_ok() {
            let mut app =
                test::init_service(App::new().wrap(middleware::Logger::default()).service(api))
                    .await;
            let req = test::TestRequest::post()
                .uri("/api")
                .set_json(JsonRpcRequest {
                    id: Some("1".to_owned()),
                    json_rpc: "2.0".to_owned(),
                    method: ToDoListMethod::HealthCheck,
                    params: None,
                })
                .to_request();
            let _resp: JsonRpcResponse = test::call_and_read_body_json(&mut app, req).await;
        }
    }
}

use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: String,
    pub data: Option<serde_json::Value>,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub id: Option<String>,
    pub json_rpc: String,
    pub method: ToDoListMethod,
    pub params: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub error: Option<JsonRpcError>,
    pub id: Option<String>,
    pub json_rpc: String,
    pub result: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
pub enum ToDoListMethod {
    HealthCheck,
}

#[post("/api")]
pub async fn api(req: web::Json<JsonRpcRequest>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(JsonRpcResponse {
        error: None,
        id: req.id.clone(),
        json_rpc: "2.0".to_owned(),
        result: None,
    }))
}
