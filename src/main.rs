use geoutils::Location;     // For calculating distances
use serde::Deserialize;     // For parsing JSON
use std::{env, fs};         // For command-line args and reading files

#[derive(Clone, Deserialize)]
struct City { code: String, name: String, lat: f32, lon: f32 }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <ORIGIN> <DEST>", args[0]);
        std::process::exit(1);
    }

    // Load cities from JSON
    let cities = load_cities();

    // Find origin city by code or exit if not found
    let origin = city_lookup(&args[1], &cities).unwrap_or_else(|| {
        eprintln!("Invalid city code: {}", args[1]); std::process::exit(1)
    });

    // Find destination city by code or exit if not found
    let dest = city_lookup(&args[2], &cities).unwrap_or_else(|| {
        eprintln!("Invalid city code: {}", args[2]); std::process::exit(1)
    });

    println!("Origin: {}, Destination: {}", origin.name, dest.name);

    // Calculate distance using Haversine formula
    let dist = calc_distance(origin.lat, origin.lon, dest.lat, dest.lon);
    println!("Distance: {:.2} km", dist);
}

// Load cities from cities.json
fn load_cities() -> Vec<City> {
    let data = fs::read_to_string("cities.json").expect("Failed to read cities.json");
    serde_json::from_str(&data).expect("Invalid JSON")
}

// Lookup city by code
fn city_lookup(code: &str, cities: &[City]) -> Option<City> {
    cities.iter().find(|c| c.code == code).cloned()
}

// Calculate distance between two coordinates in km
fn calc_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f64 {
    Location::new(lat1, lon1).haversine_distance_to(&Location::new(lat2, lon2)).meters() / 1000.0
}
