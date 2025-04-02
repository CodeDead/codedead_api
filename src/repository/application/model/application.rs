use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Application {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub name: String,
    pub description: Option<String>,
    pub platforms: Option<Vec<ApplicationPlatform>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationPlatform {
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub architectures: Option<Vec<Architecture>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Architecture {
    pub name: String,
    pub url: String,
    pub releases: Option<Vec<Release>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Release {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: Option<ReleaseType>,
    pub semver: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "infoUrl")]
    pub info_url: String,
    pub checksum: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ReleaseType {
    Major,
    Minor,
    Patch,
    PreRelease,
    Other,
}
