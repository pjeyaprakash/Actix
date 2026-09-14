pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/auth.rs"));
}

pub mod user {
    include!(concat!(env!("OUT_DIR"), "/user.get_list.rs"));
}