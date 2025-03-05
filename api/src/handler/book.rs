use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use registry::AppRegistry;
use thiserror::Error;
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};


pub async fn register_book(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}
pub async fn show_book_list(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}
pub async fn show_book(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}
