use crate::config::server_config::ServerConfig;
use crate::repository::application::dao::application_repository::ApplicationRepository;
use crate::services::Services;
use crate::services::application::application_service::ApplicationService;
use log::info;
use mongodb::Client;
use std::env;

pub struct EnvReader {}

impl EnvReader {
    /// Initialize a new EnvReader
    ///
    /// # Returns
    ///
    /// A new instance of EnvReader
    pub fn new() -> Self {
        EnvReader {}
    }

    /// Reads the server configuration from environment variables
    ///
    /// # Returns
    ///
    /// A new instance of ServerConfig
    pub async fn read_server_config(&self) -> ServerConfig {
        info!("Reading configuration from environment variables");

        let addr = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));
        let port = match env::var("SERVER_PORT") {
            Ok(d) => {
                let res: u16 = d.trim().parse().expect("SERVER_PORT must be a valid u16");
                res
            }
            Err(_) => 80,
        };

        let database_name = match env::var("MONGODB_DATABASE_NAME") {
            Ok(d) => d,
            Err(_) => panic!("MONGODB_DATABASE_NAME has not been specified"),
        };

        let connection_string = match env::var("MONGODB_CONNECTION_STRING") {
            Ok(d) => d,
            Err(_) => panic!("MONGODB_CONNECTION_STRING has not been specified"),
        };

        let client = Client::with_uri_str(connection_string)
            .await
            .expect("Failed to initialize MongoDB client");

        let max_fetch_limit = match env::var("MAX_FETCH_LIMIT") {
            Ok(d) => {
                let res: i64 = d.trim().parse().expect("MAX_FETCH_LIMIT must be a number");
                res
            }
            Err(_) => 100,
        };

        let application_collection = match env::var("APPLICATIONS_COLLECTION") {
            Ok(e) => e,
            Err(_) => {
                panic!("APPLICATIONS_COLLECTION has not been specified")
            }
        };

        let application_repository = ApplicationRepository::new(application_collection);
        let application_service = ApplicationService::new(application_repository);

        let services = Services::new(application_service);

        ServerConfig::new(
            &addr,
            port,
            &database_name,
            client,
            max_fetch_limit,
            services,
        )
    }
}
