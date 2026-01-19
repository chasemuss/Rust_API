use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;

#[derive(Deserialize)]
struct CalcResponse {
    a: f64,
    symbol: String,
    b: f64,
    result: f64,
}


#[get("/{operation}/{a}/{b}")]
async fn calculate(path: web::Path<(String, f64, f64)>) -> impl Responder {
    fn op_symbol(operation: & str) -> &'static str {
        match operation.to_lowercase().as_str() {
            "add"      => "+",
            "subtract" => "-",
            "multiply" => "*",
            "divide"   => "/",
            _          => "?",
        }
    }
    
    let (operation, a, b) = path.into_inner();
    println!("Calculating {} {} {}", a, op_symbol(&operation), b);

    let result = match operation.to_lowercase().as_str() {
        "add"      => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide"   => {
            if b == 0.0 {
                return HttpResponse::BadRequest().body("Division by zero is not permitted");
            }
            a / b
        }
        _ => {
            return HttpResponse::BadRequest().body(format!("Unknown Operation: '{}'. Supported: add, subtract, multiply, divide", operation))
        }
    };

    let symbol = op_symbol(&operation).to_string();

    let response = CalcResponse{
        a: a, 
        symbol: symbol,
        b: b, 
        result: result
    };

    println!("{} {} {} = {}", a, symbol, b, result);
    HttpResponse::Ok().json(response)
}



#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello from your Rust Calculator API!");
    HttpResponse::Ok().body("Hello from your Rust Calculator API! 🚀\n\
         Try examples:\n\
         - /add/5/3\n\
         - /subtract/10/4\n\
         - /divide/100/0 (will error)")
}

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