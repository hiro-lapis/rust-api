use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        checkout::{
            event::{CreateCheckout, UpdateReturned},
            Checkout,
        },
        id::{BookId, CheckoutId, UserId},
    },
    repository::checkout::CheckoutRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{
    model::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow},
    ConnectionPool,
};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;
        self.set_transaction_serializable(&mut tx).await?;

        // check
        let res = sqlx::query_as!(
            CheckoutStateRow,
            r#"
                SELECT
                b.book_id,
                c.checkout_id AS "checkout_id?: CheckoutId",
                NULL AS "user_id?: UserId"
                FROM books AS b
                LEFT OUTER JOIN checkouts AS c USING(book_id)
                WHERE book_id = $1;
            "#,
            event.book_id as _
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        match res {
            None => {
                return Err(AppError::EntityNotFound(format!(
                    "The book ({}) is not found",
                    event.book_id
                )))
            }
            Some(CheckoutStateRow {
                checkout_id: Some(_),
                ..
            }) => {
                return Err(AppError::UnprocessableEntity(format!(
                    "The book ({}) is already in rent",
                    event.book_id
                )))
            }
            _ => {} // otherwise, continue this func
        }

        let checkout_id = CheckoutId::new();
        let res = sqlx::query!(
            r#"
                INSERT INTO checkouts
                (checkout_id, book_id, user_id, checked_out_at)
                VALUES ($1, $2, $3, $4)
                ;
            "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "No checkout record has been created".into(),
            ));
        }

        // commit is failed, transaction will be roll backed and closed automatically
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        self.set_transaction_serializable(&mut tx).await?;

        let res = sqlx::query_as!(
            CheckoutStateRow,
            r#"
                SELECT
                b.book_id,
                c.checkout_id AS "checkout_id?: CheckoutId",
                NULL AS "user_id?: UserId"
                FROM books AS b
                LEFT OUTER JOIN checkouts AS c USING(book_id)
                WHERE book_id = $1;
        "#,
            event.book_id as _,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        match res {
            None => {
                return Err(AppError::EntityNotFound(format!("
                The book ({}) is not found
                ",
                    event.book_id,
                )))
            }
            Some(CheckoutStateRow {
                checkout_id: Some(c),
                user_id: Some(u),
                ..
            // use if expression as match condition
            }) if (c, u) != (event.checkout_id, event.returned_by) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "The checkout (ID {}), user({}), book ({}) cannot return.",
                        event.checkout_id,
                        event.returned_by,
                        event.book_id,
                    )))
                }
            _ => {}
        }

        let res = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts
                (checkout_id, book_id, user_id, checked_out_at, returned_at)
                SELECT checkout_id, book_id, user_id, checked_out_at, $2
                FROM checkouts
                WHERE checkout_id = $1
            "#,
            event.checkout_id as _,
            event.returned_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "
            No returning record has been updated"
                    .into(),
            ))?;
        }
        Ok(())
    }
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {
        sqlx::query_as!(
            CheckoutRow,
            r#"
            SELECT
                c.checkout_id,
                c.book_id,
                c.user_id,
                c.checked_out_at,
                b.title,
                b.author,
                b.isbn
            FROM checkouts AS c
            INNER JOIN books AS b USING(book_id)
            ORDER BY c.checked_out_at ASC;
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::SpecificOperationError)
    }

    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        sqlx::query_as!(
            CheckoutRow,
            r#"
            SELECT
                c.checkout_id,
                c.book_id,
                c.user_id,
                c.checked_out_at,
                b.title,
                b.author,
                b.isbn
            FROM checkouts AS c
            INNER JOIN books AS b USING(book_id)
            WHERE c.user_id = $1
            ORDER BY c.checked_out_at ASC;
            "#,
            user_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::SpecificOperationError)
    }
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        let checkout: Option<Checkout> = self.find_returned_by_book_id(book_id).await?;

        let mut checkout_histories: Vec<Checkout> = sqlx::query_as!(
            ReturnedCheckoutRow,
            r#"
                SELECT
                    rc.checkout_id,
                    rc.book_id,
                    rc.user_id,
                    rc.checked_out_at,
                    rc.returned_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM returned_checkouts AS rc
                INNER JOIN books AS b
                    USING(book_id)
                WHERE rc.book_id = $1
                ORDER BY rc.checked_out_at DESC
            "#,
            book_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .map(Checkout::from)
        .collect();

        if let Some(co) = checkout {
            checkout_histories.insert(0, co)
        }
        Ok(checkout_histories)
    }
}

impl CheckoutRepositoryImpl {
    // as the name represents, set trans level to highest level
    // serializable doesn't allow dirty read, nor fuzzy read
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<()> {
        sqlx::query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut **tx)
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn find_returned_by_book_id(&self, book_id: BookId) -> AppResult<Option<Checkout>> {
        let res = sqlx::query_as!(
            CheckoutRow,
            r#"
                    SELECT
                        c.checkout_id,
                        c.book_id,
                        c.user_id,
                        c.checked_out_at,
                        b.title,
                        b.author,
                        b.isbn
                        FROM checkouts AS c
                        INNER JOIN books AS b
                            USING(book_id)
                        WHERE c.book_id = $1
                "#,
            book_id as _,
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .map(Checkout::from);

        Ok(res)
    }
}
