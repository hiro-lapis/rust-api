use crate::model::id::{BookId, CheckOutId, UserId};
use chrono::{DateTime, Utc};
use derive_new::new;


#[derive(new)]
pub struct CreateCheckout {
    pub book_id: BookId,
    pub checkout_out_by: UserId,
    pub checkout_out_at: DateTime<Utc>,
}

#[derive(new)]
pub struct UpdateReturned {
    pub checkout_id: CheckOutId,
    pub book_id: BookId,
    pub returned_by: UserId,
    pub returned_out_at: DateTime<Utc>,
}
