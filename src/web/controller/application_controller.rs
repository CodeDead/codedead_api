use crate::config::server_config::ServerConfig;
use crate::errors::internal_server_error::InternalServerError;
use crate::web::dto::application::application_dto::ApplicationDto;
use actix_web::{HttpResponse, get, web};

#[get("/{id}")]
pub async fn find_by_id(path: web::Path<String>, pool: web::Data<ServerConfig>) -> HttpResponse {
    let id = path.into_inner();

    let res = match pool
        .services
        .application_service
        .get_by_id(&id, &pool.client.database(&pool.database_name))
        .await
    {
        Ok(app) => app,
        Err(e) => {
            log::error!("Error fetching application: {}", e);
            return HttpResponse::InternalServerError().json(InternalServerError::new(&format!(
                "Error fetching application with ID {}",
                id
            )));
        }
    };

    if res.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let dto = ApplicationDto::from(res.unwrap());
    HttpResponse::Ok().json(dto)
}

#[get("/")]
pub async fn find_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}
