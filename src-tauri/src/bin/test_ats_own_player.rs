use reqwest::Client;
use serde::Deserialize;

const LIVE_AREA_URL: &str =
    "https://tracker.ets2map.com/v3/area";

const OWN_TRUCKERSMP_ID: u64 = 2104825;
const TRACKER_SERVER_ID: i64 = 10;

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

    #[serde(rename = "X")]
    x: f64,

    #[serde(rename = "Y")]
    y: f64,

    #[serde(rename = "MpId")]
    mp_id: u64,

    #[serde(rename = "ServerId")]
    server_id: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
struct TestArea {
    name: &'static str,
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

#[tokio::main]
async fn main() {
    println!("RoadWatch ATS Own Player Test");
    println!("==============================");
    println!();
    println!(
        "Looking for TruckersMP ID: {}",
        OWN_TRUCKERSMP_ID
    );
    println!(
        "Tracker server ID: {}",
        TRACKER_SERVER_ID
    );
    println!();

    let areas = [
        TestArea {
            name: "Current RoadWatch ATS area",
            x1: -150000,
            y1: 80000,
            x2: 10000,
            y2: -50000,
        },
        TestArea {
            name: "Large ATS area",
            x1: -250000,
            y1: 150000,
            x2: 100000,
            y2: -150000,
        },
        TestArea {
            name: "Very large ATS area",
            x1: -500000,
            y1: 500000,
            x2: 500000,
            y2: -500000,
        },
    ];

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .expect("Failed to create HTTP client");

    for area in areas {
        println!("Testing area: {}", area.name);
        println!(
            "X: {} -> {} | Y: {} -> {}",
            area.x1,
            area.x2,
            area.y1,
            area.y2
        );

        let response = client
            .get(LIVE_AREA_URL)
            .query(&[
                ("x1", area.x1),
                ("y1", area.y1),
                ("x2", area.x2),
                ("y2", area.y2),
                ("server", TRACKER_SERVER_ID),
            ])
            .header("User-Agent", "RoadWatch/0.1")
            .send()
            .await;

        let response = match response {
            Ok(response) => response,

            Err(error) => {
                println!(
                    "Request failed: {}",
                    error
                );
                println!();

                continue;
            }
        };

        let status = response.status();

        println!("HTTP status: {}", status);

        if !status.is_success() {
            println!();

            continue;
        }

        let payload: AreaResponse =
            match response.json().await {
                Ok(payload) => payload,

                Err(error) => {
                    println!(
                        "JSON parse failed: {}",
                        error
                    );
                    println!();

                    continue;
                }
            };

        println!("Success: {}", payload.success);
        println!(
            "Players returned: {}",
            payload.data.len()
        );

        if let Some(player) = payload
            .data
            .iter()
            .find(|player| {
                player.mp_id == OWN_TRUCKERSMP_ID
            })
        {
            println!();
            println!("================================");
            println!("PLAYER FOUND!");
            println!("================================");
            println!(
                "Name: {}",
                player.name
            );
            println!(
                "MP ID: {}",
                player.mp_id
            );
            println!(
                "Position: X={} Y={}",
                player.x,
                player.y
            );
            println!(
                "Tracker Server ID: {:?}",
                player.server_id
            );

            return;
        }

        println!(
            "Own player was not found in this area."
        );
        println!();
    }

    println!("================================");
    println!("RESULT");
    println!("================================");
    println!(
        "Own player ID {} was not returned by the ETS2Map tracker.",
        OWN_TRUCKERSMP_ID
    );
    println!(
        "This indicates that the issue is likely not the RoadWatch ATS area."
    );
}