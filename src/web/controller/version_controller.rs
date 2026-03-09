use crate::config::server_config::ServerConfig;
use crate::errors::internal_server_error::InternalServerError;
use crate::web::dto::application::applications_query::ApplicationsQuery;
use crate::web::dto::version::version_dto::VersionDto;
use actix_web::{HttpResponse, get, web};
use log::error;

/// Fetches a Version by its ID.
///
/// # Arguments
///
/// * `id` - The ID of the Version to fetch.
///
/// # Returns
///
/// * `HttpResponse` - The HTTP response containing the Version data or an error message.
///
/// # Errors
///
/// * Returns an HTTP 404 Not Found if the Version is not found.
/// * Returns an HTTP 500 Internal Server Error if there is an error fetching the Version.
#[utoipa::path(
    get,
    path = "/api/v1/version/{id}",
    tag = "Version",
    params(
        ("id" = String, Path, description = "The ID of the Version to fetch", nullable = false),
    ),
    responses(
            (status = 200, description = "HTTP OK", body = VersionDto),
            (status = 404, description = "HTTP Not Found"),
            (status = 500, description = "HTTP Internal Server Error", body = InternalServerError),
    ),
)]
#[get("/{id}")]
pub async fn find_version_by_id(
    id: web::Path<String>,
    pool: web::Data<ServerConfig>,
) -> HttpResponse {
    let id = id.into_inner();

    let res = match pool
        .services
        .version_service
        .find_by_id(&id, &pool.client.database(&pool.database_name))
        .await
    {
        Ok(app) => app,
        Err(e) => {
            log::error!("Error fetching version: {}", e);
            return HttpResponse::InternalServerError().json(InternalServerError::new(&format!(
                "Error fetching version with ID {}",
                id
            )));
        }
    };

    if res.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let dto = VersionDto::from(res.unwrap());
    HttpResponse::Ok().json(dto)
}

/// Fetches all version with pagination support.
///
/// # Arguments
///
/// * `query` - The query parameters for pagination.
///
/// # Returns
///
/// * `HttpResponse` - The HTTP response containing a list of version or an error message.
///
/// # Errors
///
/// * Returns an HTTP 404 Not Found if no version are found.
/// * Returns an HTTP 500 Internal Server Error if there is an error fetching the version.
#[utoipa::path(
    get,
    path = "/api/v1/version/",
    tag = "Version",
    params(
        ("page" = Option<String>, Query, description = "The page", nullable = true),
        ("limit" = Option<i64>, Query, description = "The limit of the amount of entities to retrieve", nullable = true),
    ),
    responses(
            (status = 200, description = "HTTP OK", body = Vec<VersionDto>),
            (status = 404, description = "HTTP Not Found"),
            (status = 500, description = "HTTP Internal Server Error", body = InternalServerError),
    ),
)]
#[get("/")]
pub async fn find_all_versions(
    query: web::Query<ApplicationsQuery>,
    pool: web::Data<ServerConfig>,
) -> HttpResponse {
    let limit = query.limit;

    let mut new_limit = limit.unwrap_or(pool.max_fetch_limit);
    if new_limit > pool.max_fetch_limit {
        new_limit = pool.max_fetch_limit;
    }

    let res;
    if query.page.is_none() {
        res = match pool
            .services
            .version_service
            .get_all_sorted_by_id(Some(new_limit), &pool.client.database(&pool.database_name))
            .await
        {
            Ok(r) => r,
            Err(err) => {
                error!("Error fetching versions: {}", err);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new("Error fetching versions"));
            }
        };
    } else {
        res = match pool
            .services
            .version_service
            .get_all_with_id_greater_than(
                &query.page.clone().unwrap(),
                Some(new_limit),
                &pool.client.database(&pool.database_name),
            )
            .await
        {
            Ok(r) => r,
            Err(err) => {
                error!("Error fetching versions: {}", err);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new("Error fetching versions"));
            }
        };
    }

    if res.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let last_id = res
        .last()
        .map(|app| app.id.clone())
        .unwrap_or(String::from(""));

    let mut dtos: Vec<VersionDto> = vec![];
    for app in res {
        dtos.push(VersionDto::from(app));
    }

    let next;
    if dtos.len() < usize::try_from(new_limit).unwrap() {
        next = format!(
            "<{}/api/v1/version/?limit={}>; rel=first",
            &pool.server_context, new_limit
        );
    } else {
        next = format!(
            "<{}/api/v1/version/?limit={}>; rel=first, <{}/api/v1/version/?page={}&limit={}>; rel=next",
            &pool.server_context, new_limit, &pool.server_context, last_id, new_limit
        );
    }

    HttpResponse::Ok().append_header(("Link", next)).json(dtos)
}
