use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
mod database;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*dotenv().ok();
    database::migrate().await.expect("Failed to migrate database");*/
    HttpServer::new(|| {
        App::new()
            .service(index)
    }).bind(("localhost", 8080))?.run().await
}
