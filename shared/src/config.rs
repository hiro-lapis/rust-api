use anyhow::Result;

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST").unwrap(),
            port: std::env::var("DATABASE_PORT")
                .unwrap()
                .parse::<u16>()
                .unwrap(),
            username: std::env::var("DATABASE_USERNAME").unwrap(),
            password: std::env::var("DATABASE_PASSWORD").unwrap(),
            database: std::env::var("DATABASE_NAME").unwrap(),
        };

        let auth = AuthConfig {
            ttl: std::env::var("AUTH_TOLEN_TTL")?.parse::<u64>()?,
        };

        let redis = RedisConfig {
            host: std::env::var("REDIS_HOST")?,
            port: std::env::var("REDIS_PORT")?.parse::<u16>()?,
        };

        Ok(Self {
            auth,
            database,
            redis,
        })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

pub struct AuthConfig {
    pub ttl: u64,
}
