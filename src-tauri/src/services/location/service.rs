use crate::models::Location;

use super::models::{
    City,
    Game,
    LocationResult,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AtsCityData {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "countryToken")]
    country_token: String,

    x: f64,

    y: f64,
}

pub struct LocationService {
    ets2_cities: Vec<City>,
    ats_cities: Vec<City>,
}

impl LocationService {
    /// Creates a new location service and loads all available city data.
    pub fn new() -> Self {
        Self {
            ets2_cities: Self::load_ets2_cities(
                include_str!("data/ets2-cities.json"),
            ),

            ats_cities: Self::load_ats_cities(
                include_str!("data/ats-cities.json"),
            ),
        }
    }

    /// Loads ETS2 city data.
    fn load_ets2_cities(json: &str) -> Vec<City> {
        serde_json::from_str(json).unwrap_or_else(|error| {
            eprintln!(
                "RoadWatch: Failed to load ETS2 cities: {error}"
            );

            Vec::new()
        })
    }

    /// Loads ATS city data.
    ///
    /// The current ATS city file is a JSON array containing
    /// city objects with X/Y coordinates.
    fn load_ats_cities(json: &str) -> Vec<City> {
        let cities: Vec<AtsCityData> =
            match serde_json::from_str(json) {
                Ok(cities) => cities,

                Err(error) => {
                    eprintln!(
                        "RoadWatch: Failed to load ATS cities: {error}"
                    );

                    return Vec::new();
                }
            };

        let normalized_cities = cities
            .into_iter()
            .map(|city| City {
                name: city.name,

                region: Self::format_region(
                    &city.country_token,
                ),

                country: "United States".to_string(),

                x: city.x,

                y: city.y,
            })
            .collect::<Vec<_>>();

        println!(
            "RoadWatch: {} ATS cities loaded.",
            normalized_cities.len()
        );

        normalized_cities
    }

    /// Converts values such as "california" into "California"
    /// and "new_mexico" into "New Mexico".
    fn format_region(value: &str) -> String {
        value
            .split('_')
            .map(|part| {
                let mut characters = part.chars();

                match characters.next() {
                    Some(first) => {
                        first
                            .to_uppercase()
                            .collect::<String>()
                            + characters.as_str()
                    }

                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Resolves an X/Y game position to the RoadWatch
    /// Location model.
    pub fn resolve(
        &self,
        game: Game,
        x: f64,
        y: f64,
    ) -> Location {
        let cities = match game {
            Game::Ets2 => &self.ets2_cities,
            Game::Ats => &self.ats_cities,
        };

        let Some(city) =
            self.find_nearest_city(cities, x, y)
        else {
            return Location {
                city: "Unknown".to_string(),
                region: None,
                country: "Unknown".to_string(),
            };
        };

        Location {
            city: city.name.clone(),

            region: Some(city.region.clone()),

            country: city.country.clone(),
        }
    }

    /// Resolves an X/Y position and returns the extended
    /// LocationResult.
    pub fn resolve_result(
        &self,
        game: Game,
        x: f64,
        y: f64,
    ) -> LocationResult {
        let cities = match game {
            Game::Ets2 => &self.ets2_cities,
            Game::Ats => &self.ats_cities,
        };

        let Some(city) =
            self.find_nearest_city(cities, x, y)
        else {
            return LocationResult::unknown();
        };

        let distance =
            Self::distance(x, y, city.x, city.y);

        LocationResult {
            city: Some(city.name.clone()),

            region: Some(city.region.clone()),

            country: Some(city.country.clone()),

            distance: Some(distance),
        }
    }

    /// Finds the nearest city to the supplied game coordinates.
    fn find_nearest_city<'a>(
        &self,
        cities: &'a [City],
        x: f64,
        y: f64,
    ) -> Option<&'a City> {
        cities.iter().min_by(|a, b| {
            let distance_a =
                Self::distance_squared(
                    x,
                    y,
                    a.x,
                    a.y,
                );

            let distance_b =
                Self::distance_squared(
                    x,
                    y,
                    b.x,
                    b.y,
                );

            distance_a
                .partial_cmp(&distance_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Calculates squared distance.
    fn distance_squared(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> f64 {
        let dx = x1 - x2;
        let dy = y1 - y2;

        dx * dx + dy * dy
    }

    /// Calculates the actual Euclidean distance.
    fn distance(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> f64 {
        Self::distance_squared(
            x1,
            y1,
            x2,
            y2,
        )
        .sqrt()
    }
}

impl Default for LocationService {
    fn default() -> Self {
        Self::new()
    }
}