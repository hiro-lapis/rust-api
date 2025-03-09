use chrono::{DateTime, Utc};
use derive_new::new;

use super::id::{BookId, UserId};

pub mod event;

#[derive(new)]
pub struct Checkout {
    pub id: String,
    pub checkout_out_by: UserId,
    pub checkout_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub book: CheckoutBook,
}


#[derive(new)]
pub struct CheckoutBook {
    pub book_id: BookId,
    pub author: String,
    pub title:  String,
    pub isbn: String,
}
