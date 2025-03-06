use crate::database::{model::book::BookRow, ConnectionPool};
use async_trait::async_trait;
use derive_new::new;
use kernel::{model::book::{event::CreateBook, Book}, repository::book::BookRepository};
use kernel::model::id::BookId;
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await
        // change sqlx::Error to AppError
        .map_err(AppError::SpecificOperationError)?;

        // make sure don't use anyhow::Ok, just use core::Ok
        Ok(())
    }
    async fn find_all(&self) -> AppResult<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(rows.into_iter().map(Book::from).collect())
    }
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                WHERE book_id = $1
            "#,
            book_id as _ // disable type check
            // NOTE: ^ is written in query_as! macro, means that this is not cast, just disable type check temporarily
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));
        let book = CreateBook {
            title: "test hiro-title".into(),
            author: "test hiro-author".into(),
            isbn: "test hiro-isbn".into(),
            description: "test hiro-description".into(),
        };

        repo.create(book).await?;
        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        let book_id = res[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = res.unwrap();

        assert_eq!(id, book_id);
        assert_eq!(title, "test hiro-title");
        assert_eq!(isbn, "test hiro-isbn");
        assert_eq!(author, "test hiro-author");
        assert_eq!(description, "test hiro-description");

        Ok(())
    }
}
