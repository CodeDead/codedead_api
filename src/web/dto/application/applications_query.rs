use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplicationsQuery {
    pub page: Option<String>,
    pub limit: Option<i64>,
}
