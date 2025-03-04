use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// before moduling
// pub async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
//     let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
//     match connection_result {
//         Ok(_) => StatusCode::OK,
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }
