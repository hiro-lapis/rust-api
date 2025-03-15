use chrono::{DateTime, Utc};

use super::id::{BookId, CheckoutId, UserId};

pub mod event;

#[derive(Debug)]
pub struct Checkout {
    pub id: CheckoutId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>, // allow null
    pub book: CheckoutBook,
}

#[derive(Debug)]
pub struct CheckoutBook {
    pub book_id: BookId,
    pub author: String,
    pub title: String,
    pub isbn: String,
}
