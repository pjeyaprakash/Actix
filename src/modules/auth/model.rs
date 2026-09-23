use serde::Serialize;

#[derive(Serialize)]
pub struct AccessClaims {
    pub sub: i32,
    pub token_type: String,
    pub iat: usize,
    pub exp: usize,
}
#[derive(Serialize)]
pub struct RefreshClaims {
    pub sub: i32,
    pub token_type: String,
    pub jti: String,
    pub iat: usize,
    pub exp: usize,
}


// pub struct LoginDetails {
//     pub id: i32
// }


