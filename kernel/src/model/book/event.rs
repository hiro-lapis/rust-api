use crate::model::id::{BookId, UserId};

#[derive(Debug)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub description: String,
    pub isbn: String,
}

#[derive(Debug)]
pub struct UpdateBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub description: String,
    pub isbn: String,
    pub requested_user: UserId,
}

#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
    pub requested_user: UserId,
}
