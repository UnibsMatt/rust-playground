use actix_web::{web, get, post, HttpRequest, HttpResponse};
use log::{info, error};
use serde::Deserialize;

#[get("/")]
pub async fn hallo() -> HttpResponse{
    println!("Ciao");
    HttpResponse::Ok().body("Ciao come va?")
}

#[derive(Deserialize, Debug)]
struct Banana {
    message: String
}

#[post("/banana")]
pub async fn banana(request: web::Json<Banana>) -> HttpResponse {
    info!("{:?}", request);
    HttpResponse::Ok().body("ciao")
}