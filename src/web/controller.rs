pub(crate) mod actuator_controller;
pub(crate) mod application_controller;

use actix_web::web;

pub(crate) struct Controller {}

impl Controller {
    /// # Summary
    ///
    /// Configure the routes for the web server.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The web server configuration.
    pub fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api/v1")
                .service(
                    web::scope("/applications")
                        .service(application_controller::find_by_id)
                        .service(application_controller::find_all),
                )
                .service(web::scope("/actuators").service(actuator_controller::health)),
        );
    }
}
