use actix_web::{web, HttpResponse};
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
                .header("error", e.to_string())
                .finish()
        })
}

pub fn traveling(info: web::Json<TravelingInfo>) -> HttpResponse {
    if info.progress > 100 {
        return HttpResponse::BadRequest()
            .header("error", "Cannot have a progress larger than 100")
            .finish();
    }

    draw_traveling(
        info.origin.as_str(),
        info.destination.as_str(),
        info.progress,
        info.class.as_str(),
    )
    .map(send_image)
    .unwrap_or_else(|e| HttpResponse::BadRequest().header("error", e).finish())
}

pub fn in_city(info: web::Json<InCityInfo>) -> HttpResponse {
    draw_in_city(info.origin.as_str(), info.class.as_str())
        .map(send_image)
        .unwrap_or_else(|e| HttpResponse::BadRequest().header("error", e).finish())
}

pub fn ping() -> HttpResponse {
    HttpResponse::from("Pong")
}
