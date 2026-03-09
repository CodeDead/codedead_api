use crate::services::application::application_service::ApplicationService;
use crate::services::version::version_service::VersionService;

pub(crate) mod application;
pub(crate) mod version;

#[derive(Clone)]
pub struct Services {
    pub application_service: ApplicationService,
    pub version_service: VersionService,
}

impl Services {
    /// Creates a new instance of `Services` with the provided `ApplicationService`
    ///
    /// # Arguments
    ///
    /// * `application_service` - An instance of `ApplicationService`
    /// * `version_service` - An instance of `VersionService`
    ///
    /// # Returns
    ///
    /// A new instance of `Services`
    pub fn new(application_service: ApplicationService, version_service: VersionService) -> Self {
        Services {
            application_service,
            version_service,
        }
    }
}
