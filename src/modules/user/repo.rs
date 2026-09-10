use deadpool_postgres::Pool;
use crate::proto::user::UserList;

// use crate::proto::
pub struct UserRepo;
use tokio_postgres::Row;
impl UserRepo {

    pub async fn get_user(pool:&Pool) -> Result<UserList, Box<dyn std::error::Error>>  {
        let db = pool.get().await?;

        Ok(
            db.query(
                r#"
                SELECT id, name
                FROM users
                "#,
                &[]
            ).await?
        )
    }
}