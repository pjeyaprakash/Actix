use deadpool_postgres::Pool;
use crate::modules::user::repo::UserRepo;
use crate::proto::user::{User, UserList};

pub struct UserService;

impl UserService {



    pub async fn get_user(pool: &Pool) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        UserRepo::get_user(pool).await
    }

}