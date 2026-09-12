use deadpool_postgres::Pool;
use futures_util::{pin_mut, TryStreamExt};

use crate::proto::user::{User, UserList};

pub struct UserRepo;

impl UserRepo {
    pub async fn get_user(
        pool: &Pool
    ) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let db = pool.get().await?;

        let stmt = db
            .prepare_cached(
                r#"
                SELECT id, name
                FROM users
                "#,
            )
            .await?;

        let stream = db
            .query_raw(
                &stmt,
                std::iter::empty::<&(dyn tokio_postgres::types::ToSql + Sync)>(),
            )
            .await?;

        pin_mut!(stream);

        let mut users = Vec::new();
        while let Some(row) = stream.try_next().await? {
            users.push(User {
                id: row.get(0),
                name: row.get(1),
            });
        }

        // Ok(UserList { data: users })
        Ok(users)
    }
}