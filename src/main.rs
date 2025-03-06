use geoutils::Location;
use std::env;

#[derive(Clone)]
struct City {
    code: String,
    name: String,
    lat: f32,
    lon: f32,
}
fn main() {
    let arguments: Vec<String> = env::args().collect();
    // dbg!(&arguments);

    if arguments.len() < 3 {
        panic!("Error Enter the codes of origin and destination");
    }

    let origin_city: City;

    if let Some(city) = city_lookup(&arguments[1]) {
        origin_city = city;
        println!("Origin City entered: {0}", origin_city.name);
    } else {
        print!("Invalid city code {} ", &arguments[1]);
        std::process::exit(-1);
    }

    let destination_city: City;

    if let Some(city) = city_lookup(&arguments[2]) {
        destination_city = city;
        println!("Destination City entered: {0}", destination_city.name);
    } else {
        print!("Invalid city code {} ", &arguments[2]);
        std::process::exit(-1);
    }

    let dist = calc_distance(
        origin_city.lat,
        origin_city.lon,
        destination_city.lon,
        destination_city.lon,
    );

    println!("Distnace: {:.2} kilometeres", dist);
    std::process::exit(0);
}

fn city_lookup(code: &str) -> Option<City> {
    let cities: [City; 4] = [
        City {
            name: String::from("LAX"),
            code: String::from("LosAngeles"),
            lat: 34.0549,
            lon: -118.2426,
        },
        City {
            code: String::from("SFO"),
            name: String::from("San Francisco"),
            lat: 37.7749,
            lon: -122.4194,
        },
        City {
            code: String::from("SAN"),
            name: String::from("San Diego"),
            lat: 32.7157,
            lon: -117.1611,
        },
        City {
            code: String::from("PHX"),
            name: String::from("Phoneix"),
            lat: 33.4484,
            lon: -112.0740,
        },
    ];

    let result: Option<&City> = cities.iter().find(|city| city.code == code);

    match result {
        None => None,
        Some(city) => Some(city.clone()),
    }
}
fn calc_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f64 {
    let point_a = Location::new(lat1, lon1);
    let point_b = Location::new(lat2, lon2);
    return point_a.haversine_distance_to(&point_b).meters() / 1000.0;
}
