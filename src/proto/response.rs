use actix_web::HttpResponse;
use prost::Message;

pub fn protobuf_response<T: Message>(message: T) -> HttpResponse {
    let mut buf = Vec::with_capacity(message.encoded_len());

    message
        .encode(&mut buf)
        .expect("protobuf encoding failed");

    HttpResponse::Ok()
        .content_type("application/x-protobuf")
        .body(buf)
}