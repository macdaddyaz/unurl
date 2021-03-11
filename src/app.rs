use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(shorten_url)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn shorten_url() -> impl Responder {
    HttpResponse::Ok().body("abc123")
}
