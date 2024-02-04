mod model;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn status() -> impl Responder {
HttpResponse::Ok().json(model::Status { status: "UP".to_string() })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(status))
    }).bind(("127.0.0.1", 8080))?.run().await
}