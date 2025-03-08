use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::UserId,
        user::{
            event::{CreateUser, DeleteUser, UpdateUserCurrentPassword, UpdateUserRole},
            User,
        },
    },
    repository::user::UserRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{model::user::UserRow, ConnectionPool};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id,
                    u.name,
                    u.email,
                    r.name as role_name,
                    u.created_at,
                    u.updated_at
                FROM users AS u
                INNER JOIN roles as r Using(role_id)
                WHERE u.user_id = $1
            "#,
            current_user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        match row {
            Some(r) => Ok(Some(User::try_from(r)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        todo!()
        // let rows = sqlx::query_as!(
        //     UserRow,
        //     r#"
        //         SELECT
        //             u.user_id,
        //             u.name,
        //             u.email,
        //             r.name as role_name,
        //             u.created_at,
        //             u.updated_at
        //         FROM users as u
        //         INNER JOIN roles as r Using(role_id)
        //     "#,
        // )
        // .fetch_all(self.db.inner_ref())
        // .await
        // .map_err(AppError::SpecificOperationError)?;
        // Ok(rows.into_iter().map(|u| User::try_from(u))?.collect())
    }

    async fn create(&self, event: CreateUser) -> AppResult<User> {
        todo!()
        // sqlx::query!(
        //     r#"
        //         INSERT INTO users (
        //             name,
        //             email,
        //             password_hash
        //         ) VALUES (
        //          $1,
        //          $2,
        //          $3
        //         )
        //     "#,
        //     event.name,
        //     event.email,
        //     event.password_hash
        // )
        // .execute(self.db.inner_ref())
        // .await
        // .map_err(AppError::SpecificOperationError)?;

        // Ok()
    }

    async fn update_password(&self, event: UpdateUserCurrentPassword) -> AppResult<()> {
        todo!()
    }

    async fn update_role(&self, event: UpdateUserRole) -> AppResult<()> {
        todo!()
    }

    async fn delete(&self, event: DeleteUser) -> AppResult<()> {
        todo!()
    }
}
