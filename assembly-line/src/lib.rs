// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: u8 = 221;
const MINUTES: f64 = 60.0;
const P: f64 = 100.0;
const P_90: f64 = 90.0;
const P_77: f64 = 77.0;

fn percent(items: f64, &percent: &f64) -> f64 {
    items * percent / P
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let res: f64 = (speed as f64 * CARS_PER_HOUR as f64) as f64;
    let result: f64 = match speed {
        s if s >= 5 && s <= 8 => percent(res, &P_90),
        s if s >= 9 && s <= 10 => percent(res, &P_77),
        _ => res
    };
    result
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let items = (production_rate_per_hour(speed) / MINUTES) as u32;
    items
}
