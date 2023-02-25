use actix_web::{web, HttpResponse, Responder};
use image::{DynamicImage, ImageBuffer, Rgba};
use serde::Deserialize;
use std::io::Cursor;

use crate::drawer::{draw_in_city, draw_traveling};

#[derive(Deserialize)]
pub struct TravelingInfo {
    origin: String,
    destination: String,
    progress: u8,
    class: String,
}

#[derive(Deserialize)]
pub struct InCityInfo {
    origin: String,
    class: String,
}

fn send_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> HttpResponse {
    let mut bytes: Vec<u8> = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .map(|_| HttpResponse::Ok().content_type("image/png").body(bytes))
        .unwrap_or_else(|e| {
            HttpResponse::InternalServerError()
                .append_header(("error", e.to_string()))
                .finish()
        })
}

pub async fn traveling(info: web::Json<TravelingInfo>) -> impl Responder {
    if info.progress > 100 {
        return HttpResponse::BadRequest()
            .append_header(("error", "Cannot have a progress larger than 100"))
            .finish();
    }

    draw_traveling(
        info.origin.as_str(),
        info.destination.as_str(),
        info.progress,
        info.class.as_str(),
    )
    .map(send_image)
    .unwrap_or_else(|e| {
        HttpResponse::BadRequest()
            .append_header(("error", e))
            .finish()
    })
}

pub async fn in_city(info: web::Json<InCityInfo>) -> impl Responder {
    draw_in_city(info.origin.as_str(), info.class.as_str())
        .map(send_image)
        .unwrap_or_else(|e| {
            HttpResponse::BadRequest()
                .append_header(("error", e))
                .finish()
        })
}

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong")
}
