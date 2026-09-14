use actix_web::App;
use deadpool_postgres::Pool;
use crate::utils::error::AppError;
use futures_util::{pin_mut, TryStreamExt};
use std::iter::{once};

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

}