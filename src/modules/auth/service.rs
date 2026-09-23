use deadpool_postgres::Pool;
use bcrypt;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::{Uuid};
use crate::config::env::ENV;
use crate::modules::auth::model::{AccessClaims, RefreshClaims};
use crate::modules::auth::repo::AuthRepo;
use crate::proto::auth::{LoginRequest, LoginResponse, SignupRequest, SignupResponse};
use crate::utils::constants::{JWT_ACCESS_TTL, JWT_REFRESH_TTL};
use crate::utils::error::AppError;

pub struct AuthService;
pub struct JwtService;

impl AuthService {


    pub async fn login(pool: &Pool ,data: LoginRequest) -> Result<LoginResponse, AppError> {

        if let Some(hashed_password) = AuthRepo::get_password(&pool, &data.email).await? {

            let is_valid_password = bcrypt::verify(&data.password, &hashed_password)?;

            return if is_valid_password {

                if let Some(mut login_details) = AuthRepo::get_login_details(&pool, &data.email).await? {
                    let access_token = JwtService::generate_access_token(login_details.id).await?;
                    let refresh_token = JwtService::generate_refresh_token(login_details.id).await?;

                    login_details.access_token = access_token.to_string();
                    login_details.refresh_token = refresh_token.to_string();

                    Ok(
                        LoginResponse {
                            success: true,
                            message: "Login Successful".to_string(),
                            data: login_details.into()
                        }
                    )
                } else {
                    Err(AppError::Unauthorized("Invalid Password".to_string()))
                }





            } else {
                Err(AppError::Unauthorized("Invalid Password".to_string()))
            }
        }

        Err(AppError::Unauthorized("Invalid Email".to_string()))
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


impl JwtService {
    async fn generate_access_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now().timestamp() as usize;

        let claims = AccessClaims {
            sub: user_id,
            token_type: "access".to_string(),
            iat: now,
            exp: now + JWT_ACCESS_TTL,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ENV.JWT_ACCESS_SECRET.as_bytes()),
        )
    }


    async fn generate_refresh_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now().timestamp() as usize;

        let claims = RefreshClaims {
            sub: user_id,
            token_type: "access".to_string(),
            jti: Uuid::new_v4().to_string(),
            iat: now,
            exp: now + JWT_REFRESH_TTL,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ENV.JWT_REFRESH_SECRET.as_bytes()),
        )
    }
}