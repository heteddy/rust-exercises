use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub population: u32,
    pub name: String,
    latitude: f64,
    longtitude: f64,
}

impl City {
    pub fn new(name: &str, population: u32, latitude: f64, longtitude: f64) -> Self {
        City {
            name: name.to_owned(),
            population,
            latitude,
            longtitude,
        }
    }
}
