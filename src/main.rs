
// use actix_rt::*;
use actix_web::{App, HttpServer,web, Responder};


async fn status()->impl Responder{
    // web::HttpResponse::Ok().json("Server is running")
    "{\"status\":\"Server is running\"}"
}
#[actix_rt::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new().route("/",web::get().to(status))
    }).bind(("127.0.0.1",8080))?.run().await
}