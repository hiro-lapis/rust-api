use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Result, Error, Context};
use api::route::{
    health::build_health_check_routers,
    book::build_book_routers,
};
use axum::Router;
use registry::AppRegistry;
use shared::{config::AppConfig, env::{which, Environment}};
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tower_http::LatencyUnit;
use tracing::Level;

// TODO: try to implement this api
// handler
// async fn lapis() -> &'static str {
//     "hiro lapis"
// }

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let registry = AppRegistry::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::into(self))
                        .latency_unit(LatencyUnit::Millis)
            ),
        )
        .with_state(registry);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    // axum::serve(listener, app).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause.chain = ?e.error.message %e, "Unexpected error"
            )
        })
}

// TODO: move into api layer
// #[tokio::test]
// async fn health_check_works() {
//     let status_code = health_check().await;
//     assert_eq!(status_code, StatusCode::OK);
// }

// TODO: move into adapter layer
// #[sqlx::test]
// async fn health_check_db_works (pool: sqlx::PgPool) {
//     let status_code = health_check_db(State(pool)).await;
//     assert_eq!(status_code, StatusCode::OK);
// }

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };
    // set log level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());
    // set log format
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    // initialize
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}
