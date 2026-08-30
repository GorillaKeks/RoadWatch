use reqwest::Client;
use serde::Deserialize;

const LIVE_AREA_URL: &str = "https://tracker.ets2map.com/v3/area";

#[derive(Debug, Deserialize)]
struct AreaResponse {
    #[serde(rename = "Success")]
    success: bool,

    #[serde(rename = "Data", default)]
    data: Vec<TrackerPlayer>,
}

#[derive(Debug, Deserialize)]
struct TrackerPlayer {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "MpId")]
    mp_id: u64,

    #[serde(rename = "X")]
    x: f64,

    #[serde(rename = "Y")]
    y: f64,

    #[serde(rename = "ServerId")]
    server_id: Option<u64>,
}

#[tokio::main]
async fn main() {
    println!("RoadWatch ATS Live Test");
    println!("=======================");
    println!();

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .expect("Failed to create HTTP client");

    // Large test area.
    //
    // This test is used to determine whether the ETS2Map tracker
    // returns ATS players when the correct tracker/map server IDs
    // are used.
    let x1: i64 = -200_000;
    let y1: i64 = 200_000;
    let x2: i64 = 200_000;
    let y2: i64 = -200_000;

    // IMPORTANT:
    //
    // These are tracker/map IDs, not the original TruckersMP
    // server IDs.
    let servers = [
        (8_u64, "ATS Simulation"),
        (10_u64, "ATS [US] Simulation"),
        (45_u64, "ATS [US] Arcade"),
    ];

    for (tracker_server_id, server_name) in servers {
        println!(
            "Testing ATS server: {} (Tracker ID {})",
            server_name, tracker_server_id
        );

        let response = match client
            .get(LIVE_AREA_URL)
            .query(&[
                ("x1", x1),
                ("y1", y1),
                ("x2", x2),
                ("y2", y2),
                ("server", tracker_server_id as i64),
            ])
            .header("User-Agent", "RoadWatch/0.1")
            .send()
            .await
        {
            Ok(response) => response,

            Err(error) => {
                println!("Request failed: {error}");
                println!();
                continue;
            }
        };

        println!("HTTP status: {}", response.status());

        if !response.status().is_success() {
            println!("Server returned an error.");
            println!();
            continue;
        }

        let payload: AreaResponse = match response.json().await {
            Ok(payload) => payload,

            Err(error) => {
                println!("Could not parse response: {error}");

                println!();
                continue;
            }
        };

        println!("Success: {}", payload.success);

        println!("Players returned: {}", payload.data.len());

        for player in payload.data.iter().take(10) {
            println!(
                "  {} | MP ID {} | X={} Y={} | Server={:?}",
                player.name, player.mp_id, player.x, player.y, player.server_id
            );
        }

        println!();
    }

    println!("ATS live test finished.");
}
