use kernel::model::book::{event::CreateBook, Book};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // for front end, modify field name
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
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
        } = value;

        Self {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
