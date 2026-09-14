use crate::proto::auth::{LoginRequest, LoginResponse};
use crate::utils::error::AppError;

pub struct AuthService;

impl AuthService {

    pub async fn login(data: LoginRequest) -> Result<LoginResponse, AppError> {
            println!("{:?}", data);

        Ok(
            LoginResponse {
                success: true,
                message: "success".to_string()
            }
        )
    }

}