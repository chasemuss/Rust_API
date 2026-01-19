use actix_web::{ App, HttpServer };
use rust_api::{ calculate, hello };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Calculator API running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(calculate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}