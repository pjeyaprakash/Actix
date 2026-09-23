use deadpool_postgres::Pool;
use crate::utils::error::AppError;
use futures_util::{pin_mut, TryStreamExt};
use std::iter::{once};
use crate::modules::auth::model::LoginDetails;

pub struct AuthRepo;

impl AuthRepo {

    pub async fn get_password(pool: &Pool, email: &String) -> Result<Option<String>, AppError> {
        let db = pool.get().await?;

        let stmt = db.prepare_cached(
            r#"
            SELECT password
            FROM login
            WHERE email = $1
            LIMIT 1;
            "#
        ).await?;

        let stream = db.query_raw(
            &stmt,
            once(&email as &(dyn tokio_postgres::types::ToSql + Sync))
        ).await?;

        pin_mut!(stream);

        if let Some(row) = stream.try_next().await? {
            return Ok(row.get(0))
        }

        Ok(None)
    }

    pub async fn signup(pool: &Pool, email: String, password: String) -> Result<u64, AppError> {
        let db = pool.get().await?;

        let affected_row = db.execute_raw(
            r#"
            INSERT INTO login
                (email, password)
            VALUES($1, $2)
            "#,
            &vec![&email, &password]
        ).await?;

        Ok(affected_row)
    }

    pub async fn get_login_details(pool: &Pool, email: String) -> Result<Option<LoginDetails>, AppError> {
        let db = pool.get().await?;

        let stmt = db.prepare_cached(
            r#"
            SELECT id
            FROM login
            WHERE email = $1
            LIMIT 1;
            "#
        ).await?;

        let stream = db.query_raw(
            &stmt,
            &vec![&email]
        ).await?;

        pin_mut!(stream);

        if let Some(row) = stream.try_next().await? {
            return Ok(Some(
                LoginDetails {
                id: row.get(0)
            })
            )
        }

        Ok(None)
    }

}