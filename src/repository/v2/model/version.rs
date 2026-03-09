use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub semver: String,
    pub platforms: Vec<Platform>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Platform {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub portable: Option<bool>,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "infoUrl")]
    pub info_url: Option<String>,
}
