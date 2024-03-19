use actix_web::{Error, HttpRequest, HttpResponse, post, web};
use serde_json::from_slice;
use crate::common;
#[post("/api/{token}")]
pub async fn api(
    body: web::Bytes,
    _req: HttpRequest,
) -> actix_web::Result<HttpResponse, Error> {
    println!("enter to api");
    let item = from_slice::<common::MoriokaParams>(&body)?;
    println!("model: {item:?}");

    Ok(HttpResponse::Ok().json("{}"))
}