use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: String,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
    pub rabbitmq_vhost: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rabbitmq_host: env::var("RABBITMQ_HOST").unwrap_or_else(|_| "rabbitmq".to_string()),
            rabbitmq_port: env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".to_string()),
            rabbitmq_user: env::var("RABBITMQ_USER").unwrap_or_else(|_| "guest".to_string()),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD").unwrap_or_else(|_| "guest".to_string()),
            rabbitmq_vhost: env::var("RABBITMQ_VHOST").unwrap_or_else(|_| "/".to_string()),
        }
    }

    pub fn amqp_url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}/{}",
            self.rabbitmq_user,
            self.rabbitmq_password,
            self.rabbitmq_host,
            self.rabbitmq_port,
            self.rabbitmq_vhost.trim_start_matches("/")
        )
    }
}