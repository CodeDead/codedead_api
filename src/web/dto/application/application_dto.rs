use crate::repository::application::model::application::{
    Application, ApplicationPlatform, Architecture, Release, ReleaseType,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ApplicationDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub platforms: Option<Vec<ApplicationPlatformDto>>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ApplicationPlatformDto {
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub architectures: Option<Vec<ArchitectureDto>>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ArchitectureDto {
    pub name: String,
    pub url: String,
    pub releases: Option<Vec<ReleaseDto>>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ReleaseDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub portable: Option<bool>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: Option<ReleaseTypeDto>,
    pub semver: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "infoUrl")]
    pub info_url: Option<String>,
    pub checksum: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ReleaseTypeDto {
    Major,
    Minor,
    Patch,
    PreRelease,
    Other,
}

impl From<Application> for ApplicationDto {
    fn from(application: Application) -> Self {
        ApplicationDto {
            id: application.id,
            name: application.name,
            description: application.description,
            platforms: application.platforms.map(|platforms| {
                platforms
                    .into_iter()
                    .map(ApplicationPlatformDto::from)
                    .collect()
            }),
        }
    }
}

impl From<ApplicationPlatform> for ApplicationPlatformDto {
    fn from(platform: ApplicationPlatform) -> Self {
        ApplicationPlatformDto {
            platform_name: platform.platform_name,
            architectures: platform.architectures.map(|architectures| {
                architectures
                    .into_iter()
                    .map(ArchitectureDto::from)
                    .collect()
            }),
        }
    }
}

impl From<Architecture> for ArchitectureDto {
    fn from(architecture: Architecture) -> Self {
        ArchitectureDto {
            name: architecture.name,
            url: architecture.url,
            releases: architecture
                .releases
                .map(|releases| releases.into_iter().map(ReleaseDto::from).collect()),
        }
    }
}

impl From<Release> for ReleaseDto {
    fn from(release: Release) -> Self {
        let release_type = release.release_type.map(ReleaseTypeDto::from);

        ReleaseDto {
            name: release.name,
            description: release.description,
            portable: release.portable,
            release_date: release.release_date,
            release_type,
            semver: release.semver,
            download_url: release.download_url,
            info_url: release.info_url,
            checksum: release.checksum,
        }
    }
}

impl From<ReleaseType> for ReleaseTypeDto {
    fn from(release_type: ReleaseType) -> Self {
        match release_type {
            ReleaseType::Major => ReleaseTypeDto::Major,
            ReleaseType::Minor => ReleaseTypeDto::Minor,
            ReleaseType::Patch => ReleaseTypeDto::Patch,
            ReleaseType::PreRelease => ReleaseTypeDto::PreRelease,
            ReleaseType::Other => ReleaseTypeDto::Other,
        }
    }
}
