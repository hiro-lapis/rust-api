use kernel::model::{book::Book, id::BookId};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // retrive value from arg with pattern match
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            // if want to throw away fields partially, use ..
            // ..
        } = value;
        Self {
            id: book_id.into(),
            title,
            author,
            isbn,
            description,
        }
    }
}
