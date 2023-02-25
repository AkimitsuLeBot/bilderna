use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use image::{load_from_memory, RgbaImage};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cities: HashMap<String, (i64, i64)>,
    pub paths: HashMap<String, Vec<(i64, i64)>>,
}

lazy_static! {
    pub static ref CITY_CONFIG: Config = serde_json::from_reader(BufReader::new(
        File::open("assets/config.json").expect("Failed to open config file")
    ))
    .expect("Failed to parse json config");
    pub static ref MAP: RgbaImage = load_from_memory(include_bytes!("../assets/map.png"))
        .expect("Failed to load map")
        .to_rgba8();
    pub static ref MAP_MASK: RgbaImage = load_from_memory(include_bytes!("../assets/map_mask.png"))
        .expect("Failed to load night map")
        .to_rgba8();
    pub static ref MAP_WINTER: RgbaImage =
        load_from_memory(include_bytes!("../assets/map_winter.png"))
            .expect("Failed to load winter map")
            .to_rgba8();
    pub static ref ARCHER_ICON: RgbaImage =
        load_from_memory(include_bytes!("../assets/archer.png"))
            .expect("Failed to load map")
            .to_rgba8();
    pub static ref KNIGHT_ICON: RgbaImage =
        load_from_memory(include_bytes!("../assets/knight.png"))
            .expect("Failed to load map")
            .to_rgba8();
    pub static ref TRAVELER_ICON: RgbaImage =
        load_from_memory(include_bytes!("../assets/traveler.png"))
            .expect("Failed to load map")
            .to_rgba8();
    pub static ref WIZARD_ICON: RgbaImage =
        load_from_memory(include_bytes!("../assets/wizard.png"))
            .expect("Failed to load map")
            .to_rgba8();
}
