// Cargo.toml
[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

  
// main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}


async fn greet(info: web::Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", info.name))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/greet", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
