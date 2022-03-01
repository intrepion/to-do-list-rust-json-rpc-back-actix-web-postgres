use actix_web::{App, HttpServer};
use to_do_list_json_rpc_back_actix_web_postgres::api::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
