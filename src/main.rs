use crate::component::env_reader::EnvReader;
use crate::config::open_api::ApiDoc;
use crate::web::controller::Controller;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod component;
mod config;
mod errors;
mod repository;
mod services;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let env_reader = EnvReader::new();
    let server_config = env_reader.read_server_config().await;

    let host = server_config.host.clone();
    let port = server_config.port;
    let workers = server_config.workers;

    let openapi = ApiDoc::openapi();
    let mut server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(
                SwaggerUi::new("/api/swagger-ui/{_:.*}")
                    .url("/api/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(actix_web::web::Data::new(server_config.clone()))
            .wrap(Cors::permissive())
            .configure(Controller::configure_routes)
    })
    .bind((host, port))
    .expect("Failed to bind server");

    if workers > 0 {
        server = server.workers(usize::try_from(workers).expect("Invalid number of workers"));
    }

    server.run().await
}
