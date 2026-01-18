use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

#[get("/{function}/{a}/{b}")]
async fn add(path: web::Path<(String, f64, f64)>) -> impl Responder {
    let (function, a, b) = path.into_inner();

    let result = match function.to_lowercase().as_str() {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => {
            if b == 0.0 {
                return HttpResponse::BadRequest().body("Division by zero is not permitted");
            }
            a / b
        }
        _ => {
            return HttpResponse::BadRequest().body(format!("Unknown Operation: '{}'. Supported: add, subtract, multiply, divide", function))
        }
    };

    HttpResponse::Ok().body(format!("{} {} {} = {}", a, function, b, result))
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from your Rust Calculator API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}