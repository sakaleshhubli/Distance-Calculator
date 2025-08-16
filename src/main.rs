// Import necessary crates
use geoutils::Location; // For calculating distances using lat/lon
use serde::Deserialize; // For deserializing JSON into Rust structs
use std::env;           // To read command-line arguments
use std::fs;            // To read files

// Define the City struct
// Clone allows copying City instances easily
// Deserialize allows creating City from JSON
#[derive(Clone, Deserialize)]
struct City {
    code: String,  // IATA code of the city/airport
    name: String,  // Full name of the city
    lat: f32,      // Latitude of the city
    lon: f32,      // Longitude of the city
}

fn main() {
    // Collect command-line arguments into a vector
    let arguments: Vec<String> = env::args().collect();

    // Check if user provided both origin and destination codes
    if arguments.len() < 3 {
        eprintln!("Usage: {} <ORIGIN_CODE> <DEST_CODE>", arguments[0]);
        std::process::exit(1); // Exit if not enough arguments
    }

    // Load all cities from the JSON file
    let cities = load_cities();

    // Get origin and destination codes from arguments
    let origin_code = &arguments[1];
    let dest_code = &arguments[2];

    // Lookup the origin city in the list of cities
    let origin_city = match city_lookup(origin_code, &cities) {
        Some(city) => {
            println!("Origin: {}", city.name); // Print the origin city name
            city
        }
        None => {
            eprintln!("Invalid city code: {}", origin_code);
            std::process::exit(1); // Exit if code not found
        }
    };

    // Lookup the destination city in the list of cities
    let destination_city = match city_lookup(dest_code, &cities) {
        Some(city) => {
            println!("Destination: {}", city.name); // Print destination name
            city
        }
        None => {
            eprintln!("Invalid city code: {}", dest_code);
            std::process::exit(1); // Exit if code not found
        }
    };

    // Calculate distance between the two cities using their lat/lon
    let dist = calc_distance(
        origin_city.lat,
        origin_city.lon,
        destination_city.lat,
        destination_city.lon,
    );

    // Print the distance in kilometers with 2 decimal points
    println!("Distance: {:.2} kilometers", dist);
}

// Function to load cities from the "cities.json" file
fn load_cities() -> Vec<City> {
    // Read the file contents as a string
    let data = fs::read_to_string("cities.json")
        .expect("Failed to read cities.json");
    
    // Parse JSON into a vector of City structs
    serde_json::from_str(&data).expect("Invalid JSON format in cities.json")
}

// Function to find a city by its IATA code
fn city_lookup(code: &str, cities: &[City]) -> Option<City> {
    // Iterate through cities and clone the matching city if found
    cities.iter().find(|city| city.code == code).cloned()
}

// Function to calculate distance between two points using Haversine formula
fn calc_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f64 {
    let point_a = Location::new(lat1, lon1); // Create Location for origin
    let point_b = Location::new(lat2, lon2); // Create Location for destination
    point_a.haversine_distance_to(&point_b).meters() / 1000.0 // Convert meters to km
}
