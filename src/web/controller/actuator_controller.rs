use actix_web::{HttpResponse, get};

/// # Summary
///
/// Check the health of the application.
///
/// # Returns
///
/// * `HttpResponse` - An HTTP response indicating the health status of the application.
#[utoipa::path(
    get,
    path = "/api/v1/actuators/health",
    tag = "Actuators",
    responses(
            (status = 200, description = "HTTP OK"),
    ),
)]
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
