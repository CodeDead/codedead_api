use crate::services::Services;
use mongodb::Client;

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub client: Client,
    pub max_fetch_limit: i64,
    pub workers: i64,
    pub services: Services,
    pub server_context: String,
}

impl ServerConfig {
    /// Initialize a new ServerConfig
    ///
    /// # Arguments
    ///
    /// * `host` - The host of the server
    /// * `port` - The port of the server
    /// * `database_name` - The name of the database
    /// * `client` - The MongoDB client
    /// * `max_fetch_limit` - The maximum fetch limit
    /// * `workers` - The number of workers
    /// * `services` - The services to use
    ///
    /// # Returns
    ///
    /// A new instance of ServerConfig
    pub fn new(
        host: &str,
        port: u16,
        database_name: &str,
        client: Client,
        max_fetch_limit: i64,
        workers: i64,
        services: Services,
        server_context: &str,
    ) -> Self {
        ServerConfig {
            host: host.to_string(),
            port,
            database_name: database_name.to_string(),
            client,
            max_fetch_limit,
            workers,
            services,
            server_context: server_context.to_string(),
        }
    }
}
