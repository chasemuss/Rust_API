use actix_web::{test, App};
use rust_api::{hello, calculate, CalcResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_calculate_add() {
        let app = test::init_service(
            App::new()
                .service(hello)
                .service(calculate)
        ).await;

        let req = test::TestRequest::get()
            .uri("/calculate?operation=add&a=5&b=3")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: CalcResponse = test::read_body_json(resp).await;
        assert_eq!(body.result, 8.0);
        assert_eq!(body.symbol, "+");
    }


}