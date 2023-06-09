use actix_web::{Responder, HttpRequest, HttpResponse};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn health_check(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}