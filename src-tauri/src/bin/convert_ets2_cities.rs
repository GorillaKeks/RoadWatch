use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

const SOURCE_FILE: &str = r"C:\Projekte\RoadSync\MapData\ETS2\europe-cities.json";

const OUTPUT_FILE: &str = "src/services/location/data/ets2-cities.json";

#[derive(Debug, Deserialize)]
struct SourceCity {
    name: String,

    #[serde(rename = "countryToken")]
    country_token: String,

    x: f64,
    y: f64,
}

#[derive(Debug, Serialize)]
struct RoadWatchCity {
    name: String,
    region: String,
    country: String,
    x: f64,
    y: f64,
}

fn main() {
    println!("RoadWatch ETS2 City Converter");
    println!("==============================");

    if !Path::new(SOURCE_FILE).exists() {
        eprintln!("Source file not found:");
        eprintln!("{SOURCE_FILE}");
        std::process::exit(1);
    }

    println!("Reading source file...");
    let json = fs::read_to_string(SOURCE_FILE).expect("Failed to read source file");

    let source_cities: Vec<SourceCity> =
        serde_json::from_str(&json).expect("Failed to parse source JSON");

    println!("Found {} cities.", source_cities.len());

    let cities: Vec<RoadWatchCity> = source_cities
        .into_iter()
        .map(|city| RoadWatchCity {
            name: city.name,
            region: String::new(),
            country: format_country(&city.country_token),
            x: city.x,
            y: city.y,
        })
        .collect();

    let output = serde_json::to_string_pretty(&cities).expect("Failed to serialize city data");

    let output_path = Path::new(OUTPUT_FILE);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    fs::write(output_path, output).expect("Failed to write output file");

    println!();
    println!("Conversion complete!");
    println!("Output:");
    println!("{OUTPUT_FILE}");
}

fn format_country(token: &str) -> String {
    match token {
        "uk" => "United Kingdom".to_string(),
        "germany" => "Germany".to_string(),
        "france" => "France".to_string(),
        "italy" => "Italy".to_string(),
        "spain" => "Spain".to_string(),
        "portugal" => "Portugal".to_string(),
        "netherlands" => "Netherlands".to_string(),
        "belgium" => "Belgium".to_string(),
        "luxembourg" => "Luxembourg".to_string(),
        "austria" => "Austria".to_string(),
        "switzerland" => "Switzerland".to_string(),
        "poland" => "Poland".to_string(),
        "czech_republic" => "Czech Republic".to_string(),
        "slovakia" => "Slovakia".to_string(),
        "hungary" => "Hungary".to_string(),
        "romania" => "Romania".to_string(),
        "bulgaria" => "Bulgaria".to_string(),
        "croatia" => "Croatia".to_string(),
        "slovenia" => "Slovenia".to_string(),
        "serbia" => "Serbia".to_string(),
        "montenegro" => "Montenegro".to_string(),
        "bosnia" => "Bosnia and Herzegovina".to_string(),
        "north_macedonia" => "North Macedonia".to_string(),
        "albania" => "Albania".to_string(),
        "greece" => "Greece".to_string(),
        "denmark" => "Denmark".to_string(),
        "sweden" => "Sweden".to_string(),
        "norway" => "Norway".to_string(),
        "finland" => "Finland".to_string(),
        "estonia" => "Estonia".to_string(),
        "latvia" => "Latvia".to_string(),
        "lithuania" => "Lithuania".to_string(),
        "iceland" => "Iceland".to_string(),
        "turkey" => "Turkey".to_string(),

        other => other
            .replace('_', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();

                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}
