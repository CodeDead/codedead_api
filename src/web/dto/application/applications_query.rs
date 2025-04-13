use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ApplicationsQuery {
    pub page: Option<String>,
    pub limit: Option<i64>,
}
