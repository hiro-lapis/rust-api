use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

pub mod model;

// from DatabaseConfig to PgConnectOptions
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port) // pass value
        .username(&cfg.username)
        .password(&cfg.password)
}

// wrap PgPool to avoid makingregistery depend on PgPool
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
