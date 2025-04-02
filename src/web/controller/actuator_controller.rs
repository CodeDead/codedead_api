use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
