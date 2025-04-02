use crate::services::application::application_service::ApplicationService;

pub(crate) mod application;

#[derive(Clone)]
pub struct Services {
    pub application_service: ApplicationService,
}

impl Services {
    /// Creates a new instance of `Services` with the provided `ApplicationService`
    ///
    /// # Arguments
    ///
    /// * `application_service` - An instance of `ApplicationService`
    ///
    /// # Returns
    ///
    /// A new instance of `Services`
    pub fn new(application_service: ApplicationService) -> Self {
        Services {
            application_service,
        }
    }
}
