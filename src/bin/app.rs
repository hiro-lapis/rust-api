use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Context, Result};
use api::route::{auth::build_auth_routers, v1};
use axum::{http::Method, Router};
use chrono::{Datelike, FixedOffset, Local, Timelike, Utc};
use opentelemetry::global;
use registry::AppRegistryImpl;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use std::fmt;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::{
    cors::{self, CorsLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config));

    let app = Router::new()
        .merge(v1::routes())
        .merge(build_auth_routers())
        .layer(cors())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
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
                // separate log structure with ,
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
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

    let host = std::env::var("JAEGER_HOST")?;
    let port = std::env::var("JAEGER_PORT")?;
    let end_point = format!("{host}:{port}");
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // setting of jaeger to visualize metrics
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
    .with_endpoint(end_point)
    .with_service_name("hiro-lapis api")
    .with_auto_split_batch(true) // break the batch if it exceeds the limit
    .with_max_packet_size(8192)
    .install_simple()?;

    // set log level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // local
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_timer(JapanTimeFormatter)
        .with_target(false);
        // jsonize in production
        #[cfg(not(debug_assertions))]
        let subscriber = subscriber.json();
    // initialize
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(opentelemetry)
        .try_init()?;

    Ok(())
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

struct JapanTimeFormatter;

impl FormatTime for JapanTimeFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let jp_now = Utc::now().with_timezone(FixedOffset::east_opt(9 * 3600).as_ref().unwrap());
        write!(
            w,
            "{}年{}月{}日 {}:{}:{}",
            jp_now.year(),
            jp_now.month(),
            jp_now.day(),
            jp_now.hour(),
            jp_now.minute(),
            jp_now.second()
        )
    }
}
