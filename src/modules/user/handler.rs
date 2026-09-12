use actix_web::{web, HttpResponse};
use deadpool_postgres::Pool;
use crate::modules::user::service::UserService;
use crate::proto::{
    response::protobuf_response,
    user::UserList
};

pub struct UserHandler;
//
impl UserHandler {

    pub async fn get(pool: web::Data<Pool>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let data = UserService::get_user(&pool).await?;
        let response = UserList {
            data
        };
        Ok(protobuf_response(response))
    }
}