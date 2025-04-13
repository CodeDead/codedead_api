use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::web::controller::actuator_controller::health,
        crate::web::controller::application_controller::find_all,
        crate::web::controller::application_controller::find_by_id,
    ),
    components(schemas(
        crate::errors::internal_server_error::InternalServerError,
        crate::web::dto::application::application_dto::ApplicationDto,
        crate::web::dto::application::application_dto::ApplicationPlatformDto,
        crate::web::dto::application::application_dto::ArchitectureDto,
        crate::web::dto::application::application_dto::ReleaseDto,
        crate::web::dto::application::application_dto::ReleaseTypeDto,
    ))
)]
pub struct ApiDoc;
