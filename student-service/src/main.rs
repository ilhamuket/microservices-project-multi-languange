mod config;
mod models;
mod db;
mod rabbitmq;
mod api;

use actix_web::{web, App, HttpServer};
use log::{info, error};
use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::from_env();
    
    // Create database pool
    info!("Connecting to database...");
    let pool = match db::create_pool(&config).await {
        Ok(pool) => {
            info!("Database connection established");
            pool
        },
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Setup RabbitMQ consumer
    info!("Setting up RabbitMQ consumer...");
    if let Err(e) = rabbitmq::setup_consumer(&config, pool.clone()).await {
        error!("Failed to setup RabbitMQ consumer: {}", e);
        // Continue without RabbitMQ for now
    }
    
    info!("Starting student service at http://0.0.0.0:8080");
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}