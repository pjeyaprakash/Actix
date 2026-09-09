pub mod response;

pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/auth.login.rs"));
    include!(concat!(env!("OUT_DIR"), "/auth.signup.rs"));
}