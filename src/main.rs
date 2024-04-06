use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/keepalive")]
async fn keepalive() -> impl Responder {
    let response = Response {
        message: "Server is up".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_db = repository::database::Database::new();
    let app_data = web::Data::new(user_db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(keepalive)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8082))?
        .run()
        .await
}