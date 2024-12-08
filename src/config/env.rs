use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: String,
    pub logger: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let port = env::var("SERVER_PORT").expect("Env variable is not set");
        let logger = env::var("SERVER_DEBUG").expect("Env variable is not set");

        Self { port, logger }
    }
}