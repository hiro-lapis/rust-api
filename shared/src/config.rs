use anyhow::Result;

pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST").unwrap(),
            port: std::env::var("DATABASE_PORT").unwrap().parse::<u16>().unwrap(),
            username: std::env::var("DATABASE_USERNAME").unwrap(),
            password: std::env::var("DATABASE_PASSWORD").unwrap(),
            database: std::env::var("DATABASE_NAME").unwrap(),
        };
        Ok(Self { database })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}