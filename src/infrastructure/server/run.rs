use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::adapters::routes;
use crate::config::env::Config;
use crate::infrastructure::server::web::setup_cors;

pub async fn run_server() -> std::io::Result<()> {
    let config = Config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(&config.logger)).init();
    info!("Logger initialized with level: {}", config.logger);

    info!("Starting server on port: {}", config.port);

    let bind_address = format!("127.0.0.1:{}", config.port);
    info!("Binding to address: {}", bind_address);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(setup_cors())
            .app_data(web::Data::new(crate::application::use_cases::bubble_sort::BubbleSort))
            .configure(routes::health::configure_routes)
            .configure(routes::sort::configure_routes)
    });

    match server.bind(&bind_address) {
        Ok(server) => {
            info!("Server successfully bound to {}", bind_address);
            server.run().await
        }
        Err(e) => {
            log::error!("Failed to bind server to {}: {:?}", bind_address, e);
            Err(e)
        }
    }
}
