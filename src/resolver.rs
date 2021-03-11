use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(resolve_url)))
        .bind("127.0.0.1:9000")?
        .run()
        .await
}

async fn resolve_url() -> impl Responder {
    HttpResponse::MovedPermanently()
        .header("Location", "https://google.com")
        .finish()
}
