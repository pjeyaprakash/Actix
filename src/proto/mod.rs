pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/auth.rs"));
}

pub mod user {
    include!(concat!(env!("OUT_DIR"), "/user.get_list.rs"));
}