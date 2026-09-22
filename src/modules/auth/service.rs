use std::ops::Index;
use deadpool_postgres::Pool;
use bcrypt;
use crate::modules::auth::repo::AuthRepo;
use crate::proto::auth::{LoginRequest, LoginResponse, SignupRequest, SignupResponse};
use crate::utils::error::AppError;

pub struct AuthService;

impl AuthService {

    pub async fn login(pool: &Pool ,data: LoginRequest) -> Result<LoginResponse, AppError> {

        if let Some(hashed_password) = AuthRepo::get_password(&pool, &data.email).await? {

            let is_valid_password = bcrypt::verify(&data.password, &hashed_password)?;

            return if is_valid_password {
                Ok(
                    LoginResponse {
                        success: true,
                        message: "Login Successful".to_string()
                    }
                )
            } else {
                Err(AppError::Unauthorized("Invalid Password".to_string()))
            }
        }
        
        Err(AppError::NotFound)
    }


    pub async fn signup(pool: &Pool, data: SignupRequest) -> Result<SignupResponse, AppError> {
        let email = data.email.trim().to_lowercase();
        let password = data.password.trim().to_string();
        if email.is_empty() || password.is_empty() {
            return Err(AppError::BadRequest("Invalid Credentials".into()));
        }

        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        AuthRepo::signup(&pool, email, password_hash).await?;

        Ok(SignupResponse {
            success: false,
            message: "Invalid Email".to_string()
        })

    }
}