use roadwatch_lib::services::location::{
    Game,
    LocationService,
};

fn main() {
    println!("RoadWatch Location Test");
    println!("=======================");

    let service = LocationService::new();

    println!();

    println!("ETS2 Test");
    println!("---------");

    let ets2_location = service.resolve(
        Game::Ets2,
        -39596.79296875,
        -56040.46875,
    );

    println!("City: {}", ets2_location.city);

    println!(
        "Region: {}",
        ets2_location
            .region
            .as_deref()
            .unwrap_or("Unknown")
    );

    println!(
        "Country: {}",
        ets2_location.country
    );

    println!();

    println!("ATS Test - Las Vegas");
    println!("--------------------");

    let ats_location = service.resolve(
        Game::Ats,
        -84781.5625,
        6493.26953125,
    );

    println!("City: {}", ats_location.city);

    println!(
        "Region: {}",
        ats_location
            .region
            .as_deref()
            .unwrap_or("Unknown")
    );

    println!(
        "Country: {}",
        ats_location.country
    );
}