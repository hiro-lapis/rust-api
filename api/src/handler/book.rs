use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

pub async fn register_book(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}
pub async fn show_book_list(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}
pub async fn show_book(registry: State<AppRegistry>) -> StatusCode {
    todo!()
}
