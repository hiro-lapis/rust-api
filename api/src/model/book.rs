use super::user::{BookOwner, CheckoutUser};
use chrono::{DateTime, Utc};
use derive_new::new;
use garde::Validate;
use kernel::model::{
    book::{
        event::{CreateBook, UpdateBook},
        Book, BookListOptions, Checkout,
    },
    id::{BookId, CheckoutId, UserId},
    list::PaginatedList,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

// from api's model struct to kernel's event struct
impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;

        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct UpdateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

#[derive(new)] // tuple[0]:BookId, [1]UserId
pub struct UpdateBookRequestWithUserIds(BookId, UserId, UpdateBookRequest);

impl From<UpdateBookRequestWithUserIds> for UpdateBook {
    fn from(value: UpdateBookRequestWithUserIds) -> Self {
        let UpdateBookRequestWithUserIds(
            // NOTE: use () to retrive nested value
            book_id, // make sure name this retrived tuple[0] value as bookId
            user_id, // if this user_id move before book_id, this will change value type to book_id
            UpdateBookRequest {
                // use {}
                title,
                author,
                isbn,
                description,
            },
        ) = value;

        Self {
            book_id,
            title,
            author,
            isbn,
            description,
            requested_user: user_id,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct BookListQuery {
    #[garde(range(min = 0))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,
}

const DEFAULT_LIMIT: i64 = 20;
const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedBookResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<BookResponse>,
}

impl From<PaginatedList<Book>> for PaginatedBookResponse {
    fn from(value: PaginatedList<Book>) -> Self {
        let PaginatedList {
            total,
            offset,
            limit,
            items,
        } = value;
        Self {
            total,
            limit,
            offset,
            items: items.into_iter().map(BookResponse::from).collect(),
        }
    }
}

impl From<BookListQuery> for BookListOptions {
    fn from(value: BookListQuery) -> Self {
        let BookListQuery { limit, offset } = value;
        Self { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<BookCheckoutResponse>,
}

// from kernel's model struct to api's model struct
impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner,
            checkout,
        } = value;

        Self {
            id,
            title,
            author,
            isbn,
            description,
            owner: owner.into(), // into BookOwner trait of kernel
            checkout: checkout.map(BookCheckoutResponse::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutResponse {
    pub id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<Utc>,
}

impl From<Checkout> for BookCheckoutResponse {
    fn from(value: Checkout) -> Self {
        let Checkout {
            checkout_id,
            checked_out_at,
            checked_out_by,
        } = value;
        Self {
            id: checkout_id,
            checked_out_at,
            checked_out_by: checked_out_by.into(),
        }
    }
}
