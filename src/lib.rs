use actix_web::{ get, web, HttpResponse, Responder };
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
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

const SUPPORTED_OPERATIONS: &str = "add, subtract, multiply, divide, power/pow, modulo/mod";

fn op_symbol(operation: & str) -> &'static str {
    match operation.to_lowercase().as_str() {
        "add"      => "+",
        "subtract" => "−",
        "multiply" => "×",
        "divide"   => "÷",
        "power"  | "pow" => "^",
        "modulo" | "mod" => "%",
        _          => "?",
    }
}

#[get("/calculate")]
pub async fn calculate(query: web::Query<CalcQuery>) -> impl Responder {
    
    let CalcQuery { operation, a, b } = query.into_inner();

    let result = match operation.to_lowercase().as_str() {
        "add"      => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide"   => {
            if b == 0.0 {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Division by zero is not permitted"}))
            }
            a / b
        }
        "power"  | "pow" => a.powf(b),
        "modulo" | "mod" => {
            if b == 0.0 {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Modulo by zero is not permitted"}))
            }
            a % b
        }
        _ => {
            return HttpResponse::BadRequest()
                    .json(serde_json::json!({
                        "error": format!("Unknown Operation '{}'", operation),
                        "supported": SUPPORTED_OPERATIONS
                    })
                )
        }
    };

    let symbol = op_symbol(&operation).to_string();

    let response = CalcResponse{
        a: a, 
        symbol: symbol,
        b: b, 
        result: result
    };

    HttpResponse::Ok().json(response)
}



#[get("/")]
pub async fn hello() -> impl Responder {
    println!("Hello from your Rust Calculator API!");
    HttpResponse::Ok().body("Hello from your Rust Calculator API! 🚀\n\
         Try examples:\n\
         - /add/5/3\n\
         - /subtract/10/4\n\
         - /divide/100/0 (will error)")
}