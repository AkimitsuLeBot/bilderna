use image::{load_from_memory, RgbaImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cities: HashMap<String, (i64, i64)>,
    pub paths: HashMap<String, Vec<(i64, i64)>>,
}
macro_rules! load_asset {
    ($(static $name: ident: $t: ty = $fun: expr)*) => {
        use std::sync::OnceLock;
        $(pub fn $name() -> &'static $t {
            static TMP: OnceLock<$t> = OnceLock::new();
            TMP.get_or_init(|| $fun)
        })*
    };
}

load_asset!(
    static city_config: Config = serde_json::from_reader(BufReader::new(
        File::open("assets/config.json").expect("Failed to open config file"),
    ))
    .expect("Failed to parse json config")

    static map: RgbaImage = load_from_memory(include_bytes!("../assets/map.png"))
        .expect("Failed to load map")
        .to_rgba8()
    static map_mask: RgbaImage = load_from_memory(include_bytes!("../assets/map_mask.png"))
        .expect("Failed to load night map")
        .to_rgba8()
    static map_winter: RgbaImage = load_from_memory(include_bytes!("../assets/map_winter.png"))
        .expect("Failed to load winter map")
        .to_rgba8()
    static archer_icon: RgbaImage = load_from_memory(include_bytes!("../assets/archer.png"))
        .expect("Failed to load map")
        .to_rgba8()
    static knight_icon: RgbaImage = load_from_memory(include_bytes!("../assets/knight.png"))
        .expect("Failed to load map")
        .to_rgba8()
    static traveler_icon: RgbaImage = load_from_memory(include_bytes!("../assets/traveler.png"))
        .expect("Failed to load map")
        .to_rgba8()
    static wizard_icon: RgbaImage = load_from_memory(include_bytes!("../assets/wizard.png"))
            .expect("Failed to load map")
            .to_rgba8()
);
