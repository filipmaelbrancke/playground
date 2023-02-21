use std::io::Error;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
