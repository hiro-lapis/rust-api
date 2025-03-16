use crate::{handler, model};

// avoid compile in release build
#[cfg(debug_assertions)]
#[derive(utoipa::OpenApi)]
#[openapi(
    // API's meta information
    info(
        title = "rust api",
        contact(
            name = "api development with rust",
            url = "todo",
            email = "todo"
        ),
        description = r#"just a statement"#,
    ),
    // API path hander's routes
    paths(
        handler::health::health_check,
        handler::health::health_check_db,
        // handler::book::show_book_list,
        // handler::book::show_book,
        // handler::book::register_book,
        // handler::book::update_book,
        // handler::book::delete_book,
        // handler::checkout::checkout_book,
        // handler::checkout::return_book,
        // handler::checkout::checkout_history,
        // handler::user::get_current_user,
        // handler::auth::login,
        // handler::auth::logout,
    ),
    components(schemas(
        // model::book::CreateBookRequest,
        // model::book::UpdateBookRequest,
        // model::book::BookResponse,
        // model::book::PaginatedBookResponse,
        // model::book::BookCheckoutResponse,
        // model::checkout::CheckoutsResponse,
        // model::checkout::CheckoutResponse,
        // model::checkout::CheckoutBookResponse,
        // model::user::BookOwner,
        // model::user::CheckoutUser,
        model::auth::LoginRequest,
        model::auth::AccessTokenResponse,
        // kernel::model::id::BookId,
        // kernel::model::id::UserId,
        // kernel::model::id::CheckoutId,
    ))
)]
pub struct ApiDoc;
