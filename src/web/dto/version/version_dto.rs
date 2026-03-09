use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct VersionDto {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub semver: String,
    pub platforms: Vec<PlatformDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PlatformDto {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub portable: Option<bool>,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "infoUrl")]
    pub info_url: Option<String>,
}

impl From<crate::repository::v2::model::version::Version> for VersionDto {
    /// Converts a Version from the repository model to a VersionDto for API responses.
    ///
    /// # Arguments
    ///
    /// * `version` - The Version instance from the repository model to convert.
    ///
    /// # Returns
    ///
    /// A VersionDto instance containing the data from the provided Version.
    fn from(version: crate::repository::v2::model::version::Version) -> Self {
        VersionDto {
            id: version.id,
            created_at: version.created_at,
            updated_at: version.updated_at,
            name: version.name,
            description: version.description,
            semver: version.semver,
            platforms: version
                .platforms
                .into_iter()
                .map(|p| PlatformDto {
                    name: p.name,
                    arch: p.arch,
                    portable: p.portable,
                    download_url: p.download_url,
                    info_url: p.info_url,
                })
                .collect(),
        }
    }
}
