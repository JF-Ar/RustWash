use actix_web::{App, web, get, HttpResponse, HttpServer, Responder};
use database::AppState;
mod database;
mod http;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Wash Cars - Rust API")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _poll = database::connect().await;
    database::migrate().await.expect("Fuuuuu");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgres_client: _poll.clone(),
            }))
            .service(index)
            .configure(http::customers_routes)
    }).bind(("localhost", 8081))?.run().await
}
