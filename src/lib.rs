use actix_web::{ get, web, HttpResponse, Responder };
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CalcQuery {
    operation: String,
    a: f64,
    b: f64,
}

#[derive(Deserialize, Serialize)]
pub struct CalcResponse {
    pub a: f64,
    pub symbol: String,
    pub b: f64,
    pub result: f64,
}


#[get("/calculate")]
async fn calculate(query: web::Query<CalcQuery>) -> impl Responder {
    fn op_symbol(operation: & str) -> &'static str {
        match operation.to_lowercase().as_str() {
            "add"      => "+",
            "subtract" => "-",
            "multiply" => "*",
            "divide"   => "/",
            "power"  | "pow" => "^",
            "modulo" | "mod" => "%",
            _          => "?",
        }
    }
    
    let CalcQuery { operation, a, b } = query.into_inner();

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
        "power"  | "pow" => a.powf(b),
        "modulo" | "mod" => {
            if b == 0.0 {
                return HttpResponse::BadRequest().body("Modulo by zero is not permitted")
            }
            a % b
        }
        _ => {
            return HttpResponse::BadRequest().body(format!("Unknown Operation: '{}'. Supported: add, subtract, multiply, divide", operation))
        }
    };

    let symbol = op_symbol(&operation).to_string();

    let response = CalcResponse{
        a: a, 
        symbol: symbol.clone(),
        b: b, 
        result: result
    };

    println!("{} {} {} = {}", a, symbol.clone(), b, result);
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