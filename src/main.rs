use geoutils::Location;
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Clone, Deserialize)]
struct City {
    code: String,
    name: String,
    lat: f32,
    lon: f32,
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 3 {
        eprintln!("Usage: {} <ORIGIN_CODE> <DEST_CODE>", arguments[0]);
        std::process::exit(1);
    }

    // Load all cities from JSON
    let cities = load_cities();

    let origin_code = &arguments[1];
    let dest_code = &arguments[2];

    let origin_city = match city_lookup(origin_code, &cities) {
        Some(city) => {
            println!("Origin: {}", city.name);
            city
        }
        None => {
            eprintln!("Invalid city code: {}", origin_code);
            std::process::exit(1);
        }
    };

    let destination_city = match city_lookup(dest_code, &cities) {
        Some(city) => {
            println!("Destination: {}", city.name);
            city
        }
        None => {
            eprintln!("Invalid city code: {}", dest_code);
            std::process::exit(1);
        }
    };

    let dist = calc_distance(
        origin_city.lat,
        origin_city.lon,
        destination_city.lat,
        destination_city.lon,
    );

    println!("Distance: {:.2} kilometers", dist);
}

fn load_cities() -> Vec<City> {
    let data = fs::read_to_string("cities.json")
        .expect("Failed to read cities.json");
    serde_json::from_str(&data).expect("Invalid JSON format in cities.json")
}

fn city_lookup(code: &str, cities: &[City]) -> Option<City> {
    cities.iter().find(|city| city.code == code).cloned()
}

fn calc_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f64 {
    let point_a = Location::new(lat1, lon1);
    let point_b = Location::new(lat2, lon2);
    point_a.haversine_distance_to(&point_b).meters() / 1000.0
}
