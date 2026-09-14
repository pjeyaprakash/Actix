use deadpool_postgres::Pool;
use bcrypt::verify;
use crate::modules::auth::repo::AuthRepo;
use crate::proto::auth::{LoginRequest, LoginResponse};
use crate::utils::error::AppError;

pub struct AuthService;

impl AuthService {

    pub async fn login(pool: &Pool ,data: LoginRequest) -> Result<LoginResponse, AppError> {

        if let Some(hashed_password) = AuthRepo::get_password(&pool, &data.email).await? {
            let is_valid_password = verify(&data.password, &hashed_password)
                                    .map_err(|_| AppError::InternalServerError)?;

            if is_valid_password {
                return Ok(
                    LoginResponse {
                        success: true,
                        message: "Login Successful".to_string()
                    }
                )
            } else {
                return Ok(
                    LoginResponse {
                        success: false,
                        message: "Invalid Password".to_string()
                    }
                )
            }
        }


        Ok(
            LoginResponse {
                success: false,
                message: "Email Not Found".to_string()
            }
        )
    }

}