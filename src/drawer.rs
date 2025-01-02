use chrono::{DateTime, Datelike, Timelike, Utc};
use image::imageops::overlay;
use image::{ImageBuffer, Pixel, Rgba, RgbaImage};
use std::cmp::{min, Ordering};

use crate::assets;

const NIGHT_START: f64 = 19.0;
const NIGHT_MAX: f64 = 23.0;
const DAY_START: f64 = 5.0;
const DAY_MAX: f64 = 8.0;

trait TupleMaths {
    fn distance(&self, other: &(i64, i64)) -> f64;

    fn cut(&self, other: &(i64, i64), position: f64) -> (i64, i64);
}

trait Distance {
    fn distance(&self) -> f64;
}

trait Winter {
    fn is_winter(&self) -> bool;
}

trait Night {
    fn night_progress(&self) -> f64;
}

impl TupleMaths for (i64, i64) {
    fn distance(&self, to: &(i64, i64)) -> f64 {
        (((self.0 - to.0).pow(2) + (self.1 - to.1).pow(2)) as f64).sqrt()
    }

    fn cut(&self, to: &(i64, i64), cut: f64) -> (i64, i64) {
        let x_diff = (to.0 - self.0) as f64;
        let y_diff = (to.1 - self.1) as f64;
        let steep = y_diff / x_diff;
        let nw_x = self.0 as f64 + x_diff * cut;
        let nw_y = self.1 as f64 + (x_diff * cut) * steep;
        (nw_x.round() as i64, nw_y.round() as i64)
    }
}

impl Distance for Vec<(i64, i64)> {
    fn distance(&self) -> f64 {
        self.iter()
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
    fn night_progress(&self) -> f64 {
        let current_hour = (f64::from(self.hour())) + f64::from(self.minute()) / 60.0;

        match current_hour {
            h if h >= NIGHT_MAX || h <= DAY_START => 1.0,
            h if h >= NIGHT_START => (h - NIGHT_START) / (NIGHT_MAX - NIGHT_START),
            h if h <= DAY_MAX => (DAY_MAX - h) / (DAY_MAX - DAY_START),
            _ => 0.0,
        }
    }
}

fn seasonal_map(date: DateTime<Utc>) -> RgbaImage {
    if date.is_winter() {
        assets::map_winter().clone()
    } else {
        assets::map().clone()
    }
}

fn mask_map(date: DateTime<Utc>, base_map: &mut RgbaImage) {
    let gradient_value = date.night_progress();
    if gradient_value == 0.0 {
        return;
    }

    let gradient_start = Rgba([5, 2, 6, 0]);
    let mut gradient_end = Rgba([0, 1, 3, 240]);

    let mut mask = assets::map_mask().pixels();

    gradient_end.apply2(&gradient_start, |a, b| {
        (f64::from(b) + ((f64::from(a) - f64::from(b)) * gradient_value).round()) as u8
    });
    let alpha = gradient_end.channels()[3];

    // let current_gradient = (gradient_end - gradient_start) / gradient_value;
    base_map.pixels_mut().for_each(|p| {
        let mut blend = *mask.next().unwrap();
        let prev_alpha = blend.channels()[3];
        blend.blend(&gradient_end);
        blend.channels_mut()[3] = min(alpha, prev_alpha);
        p.blend(&blend);
    });
}

fn map_image() -> RgbaImage {
    let now = Utc::now();
    let mut season = seasonal_map(now);
    mask_map(now, &mut season);
    season
}

fn class_icon(class: &str) -> &RgbaImage {
    match class {
        "ARCHER" => assets::archer_icon(),
        "KNIGHT" => assets::knight_icon(),
        "MAGE" => assets::wizard_icon(),
        _ => assets::traveler_icon(),
    }
}

/// # Errors
///
/// Will return `Err` when the origin name isn't found
pub fn draw_in_city(origin: &str, class: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let mut map = map_image();

    let config = &assets::city_config();

    let city_path = config
        .cities
        .get(origin)
        .ok_or(format!("Cannot find the city {origin}"))?;
    overlay(&mut map, class_icon(class), city_path.0, city_path.1);

    Ok(map)
}

/// # Errors
///
/// Returns and error when the path isn't found
pub fn draw_traveling(
    origin: &str,
    destination: &str,
    progress: u8,
    class: &str,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let mut map = map_image();
    let config = &assets::city_config();

    let (key, reverse) = match origin.cmp(destination) {
        Ordering::Equal => return Err(String::from("Cannot travel from and to same place")),
        Ordering::Greater => (format!("{destination}:{origin}"), true),
        Ordering::Less => (format!("{origin}:{destination}"), false),
    };

    let mut path = config
        .paths
        .get(key.as_str())
        .ok_or(format!("Cannot find the path {key}"))?
        .clone();

    if reverse {
        path.reverse();
    }
    let total = path.distance();
    let mut max_dist = total * f64::from(progress) / 100.0;

    path.iter().zip(path.iter().skip(1)).for_each(|(from, to)| {
        let dist = from.distance(to);
        if max_dist > 0.0 {
            if dist > max_dist {
                let shorten = max_dist / dist;
                let nw_to = from.cut(to, shorten);
                overlay(&mut map, class_icon(class), nw_to.0 - 16, nw_to.1 - 16);
                max_dist = 0.0;
            } else {
                max_dist -= dist;
            }
        }
    });

    Ok(map)
}
