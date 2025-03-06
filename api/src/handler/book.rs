use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use registry::AppRegistry;
use shared::error::AppError;
// use thiserror::Error;
// use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        // await result and format return value with map func
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}
pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<BookResponse>>, AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
        .map(Json) // .map(|e| Json(e))
        .map_err(AppError::from)
}
pub async fn show_book(
    Path(book_id): Path<Uuid>, // receive path parameter value
    State(registry): State<AppRegistry>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound("The specific book was not found.".to_string())),
        })
        .map_err(AppError::from)
}

// #[derive(Error, Debug)]
// pub enum AppError {
//     #[error("{0}")]
//     InternalError(#[from] anyhow::Error),
// }
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
//     }
// }
