use std::io::Error;
use actix_web::{HttpResponse, web, HttpServer, App};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), Error> {
    HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
