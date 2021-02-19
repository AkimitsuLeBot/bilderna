use crate::drawer::{draw_traveling, draw_in_city};

pub mod assets;
pub mod drawer;

fn main() {
    draw_traveling("TimmerJacka", "Jägarens", 65, "Traveler").unwrap_or_else(|e| {
        println!("{}", e);
        assets::MAP.clone()
    }).save("traveling.png").unwrap();

    draw_in_city("Jägarens", "Knight").unwrap_or_else(|e| {
        println!("{}", e);
        assets::MAP.clone()
    }).save("in_city.png").unwrap();
}