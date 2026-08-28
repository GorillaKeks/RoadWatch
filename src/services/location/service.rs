use super::models::{City, Game, LocationResult};

pub struct LocationService {
    ets2_cities: Vec<City>,
    ats_cities: Vec<City>,
}

impl LocationService {
    /// Creates a new location service and loads all available city data.
    pub fn new() -> Self {
        Self {
            ets2_cities: Self::load_cities(include_str!("data/ets2-cities.json")),
            ats_cities: Self::load_cities(include_str!("data/ats-cities.json")),
        }
    }

    /// Loads city data from a JSON string.
    fn load_cities(json: &str) -> Vec<City> {
        match serde_json::from_str::<Vec<City>>(json) {
            Ok(cities) => cities,
            Err(error) => {
                eprintln!("Failed to load city data: {error}");
                Vec::new()
            }
        }
    }

    /// Resolves a map position to the nearest known city.
    pub fn resolve(
        &self,
        game: Game,
        x: f64,
        y: f64,
    ) -> LocationResult {
        let cities = match game {
            Game::Ets2 => &self.ets2_cities,
            Game::Ats => &self.ats_cities,
        };

        let Some(city) = Self::find_nearest_city(cities, x, y) else {
            return LocationResult::unknown();
        };

        let distance = Self::distance(x, y, city.x, city.y);

        LocationResult {
            city: Some(city.name.clone()),
            region: Some(city.region.clone()),
            country: Some(city.country.clone()),
            distance: Some(distance),
        }
    }

    /// Finds the nearest city to the given coordinates.
    fn find_nearest_city(
        cities: &[City],
        x: f64,
        y: f64,
    ) -> Option<&City> {
        cities.iter().min_by(|a, b| {
            let distance_a = Self::distance_squared(x, y, a.x, a.y);
            let distance_b = Self::distance_squared(x, y, b.x, b.y);

            distance_a
                .partial_cmp(&distance_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the squared distance between two coordinates.
    fn distance_squared(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;

        dx * dx + dy * dy
    }

    /// Returns the direct distance between two coordinates.
    fn distance(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> f64 {
        Self::distance_squared(x1, y1, x2, y2).sqrt()
    }
}

impl Default for LocationService {
    fn default() -> Self {
        Self::new()
    }
}