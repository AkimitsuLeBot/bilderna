use std::cmp::Ordering;

use chrono::{Datelike, DateTime, Timelike, Utc};
use image::{ImageBuffer, Rgba, RgbaImage};
use image::imageops::overlay;

use crate::assets;

trait TupleMaths {
    fn distance(&self, other: &(i32, i32)) -> f32;

    fn cut(&self, other: &(i32, i32), position: f32) -> (i32, i32);
}

trait Distance {
    fn distance(&self) -> f32;
}

trait Winter {
    fn is_winter(&self) -> bool;
}

trait Night {
    fn is_night(&self) -> bool;
}

impl TupleMaths for (i32, i32) {
    fn distance(&self, to: &(i32, i32)) -> f32 {
        (((self.0 - to.0).pow(2) + (self.1 - to.1).pow(2)) as f32).sqrt()
    }

    fn cut(&self, to: &(i32, i32), cut: f32) -> (i32, i32) {
        let x_diff = (to.0 - self.0) as f32;
        let y_diff = (to.1 - self.1) as f32;
        let steep = y_diff / x_diff;
        let nwx = self.0 as f32 + x_diff * cut;
        let nwy = self.1 as f32 + (x_diff * cut) * steep;
        (nwx.round() as i32, nwy.round() as i32)
    }
}

impl Distance for Vec<(i32, i32)> {
    fn distance(&self) -> f32 {
        self
            .iter()
            .zip(self.iter().skip(1))
            .map(|(from, to)| from.distance(to))
            .sum()
    }
}

impl Winter for DateTime<Utc> {
    // Winter is from the 21 of December, but we'll make it a bit longer
    // and it ends the 20th of march
    fn is_winter(&self) -> bool {
        self.month() >= 12 || (self.month() < 3 || (self.month() == 3 && self.day() <= 20))
    }
}

impl Night for DateTime<Utc> {
    fn is_night(&self) -> bool {
        self.hour() >= 22 || self.hour() <= 5
    }
}

fn map_image() -> RgbaImage {
    let today = Utc::now();
    let flags = (today.is_night() as i32) + (today.is_winter() as i32) * 2;
    match flags {
        0 => assets::MAP.clone(),
        1 => assets::MAP_NIGHT.clone(),
        2 => assets::MAP_WINTER.clone(),
        _ => assets::MAP_WINTER_NIGHT.clone()
    }
}

fn class_icon(class: &str) -> &RgbaImage {
    match class {
        "ARCHER" => &assets::ARCHER_ICON as &RgbaImage,
        "KNIGHT" => &assets::KNIGHT_ICON as &RgbaImage,
        "MAGE" => &assets::WIZARD_ICON as &RgbaImage,
        _ => &assets::TRAVELER_ICON as &RgbaImage
    }
}

pub fn draw_in_city(origin: &str, class: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let mut map = map_image();

    let config = &assets::CITY_CONFIG;

    let city_path = config.cities.get(origin).ok_or(format!("Cannot find the city {}", origin))?;
    overlay(&mut map, class_icon(class), city_path.0 as u32, city_path.1 as u32);

    Ok(map)
}

pub fn draw_traveling(origin: &str, destination: &str, progress: u8, class: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let mut map = map_image();
    let config = &assets::CITY_CONFIG;

    let (key, reverse) = match origin.cmp(destination) {
        Ordering::Equal => return Err(String::from("Cannot travel from and to same place")),
        Ordering::Greater => (format!("{}:{}", destination, origin), true),
        Ordering::Less => (format!("{}:{}", origin, destination), false)
    };

    let mut path = config.paths.get(key.as_str()).ok_or(format!("Cannot find the path {}", key))?.clone();

    if reverse {
        path.reverse();
    }
    let total = path.distance();
    let mut max_dist = total * (progress as f32) / 100.0;

    path.iter().zip(path.iter().skip(1)).for_each(|(from, to)| {
        let dist = from.distance(to);
        if max_dist > 0.0 {
            if dist > max_dist {
                let shorten = max_dist / dist;
                let nw_to = from.cut(to, shorten);
                overlay(&mut map, class_icon(class), nw_to.0 as u32 - 16, nw_to.1 as u32 - 16);
                max_dist = 0.0;
            } else {
                max_dist -= dist;
            }
        }
    });

    Ok(map)
}