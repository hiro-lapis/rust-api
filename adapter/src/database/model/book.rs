use kernel::model::{
    book::{Book, Checkout},
    id::{BookId, CheckoutId, UserId},
    user::{BookOwner, CheckoutUser, User},
};
use sqlx::types::chrono::{DateTime, Utc};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}

impl BookRow {
    pub fn into_book(self, checkout: Option<Checkout>) -> Book {
        // retrive value from arg with pattern match
        let BookRow {
            // rust requires to give struct name when struct literal is used
            book_id,
            title,
            author,
            isbn,
            description,
            owned_by,
            owner_name,
            // if want to throw away fields partially, use ..
            // ..
        } = self;
        Book {
            id: book_id,
            title,
            author,
            isbn,
            description,
            owner: BookOwner {
                // rust requires to give struct name when struct literal is used
                id: owned_by,
                name: owner_name,
            },
            checkout,
        }
    }
    // fn from(value: BookRow) -> Self {
    //     // retrive value from arg with pattern match
    //     let BookRow {
    //         // rust requires to give struct name when struct literal is used
    //         book_id,
    //         title,
    //         author,
    //         isbn,
    //         description,
    //         owned_by,
    //         owner_name,
    //         // if want to throw away fields partially, use ..
    //         // ..
    //     } = value;
    //     Self {
    //         id: book_id,
    //         title,
    //         author,
    //         isbn,
    //         description,
    //         owner: BookOwner {
    //             // rust requires to give struct name when struct literal is used
    //             id: owned_by,
    //             name: owner_name,
    //         },
    //     }
    // }
}

pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}

pub struct BookCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub user_name: String,
    pub checked_out_at: DateTime<Utc>,
}

impl From<BookCheckoutRow> for Checkout {
    fn from(value: BookCheckoutRow) -> Self {
        let BookCheckoutRow {
            checkout_id,
            book_id: _,
            user_id,
            checked_out_at,
            user_name,
        } = value;

        Self {
            checkout_id,
            checked_out_by: CheckoutUser {
                id: user_id,
                name: user_name,
            },
            checked_out_at,
        }
    }
}
