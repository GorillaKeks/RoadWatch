use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use reqwest::Client;
use serde::Deserialize;

use crate::models::{
    GameType,
    LivePlayer,
    LivePosition,
    PlayerStatus,
};

use crate::services::truckersmp::server::TruckersMpServerService;

use super::config::GLOBAL_ATS_AREA;
use super::provider::LivePlayerProvider;

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

    #[serde(rename = "X")]
    x: f64,

    #[serde(rename = "Y")]
    y: f64,

    #[serde(rename = "Heading")]
    heading: Option<f64>,

    #[serde(rename = "MpId")]
    mp_id: u64,

    #[serde(rename = "PlayerId")]
    player_id: Option<u64>,

    #[serde(rename = "ServerId")]
    server_id: Option<u64>,

    #[serde(rename = "ServerType")]
    server_type: Option<u64>,

    #[serde(rename = "Time")]
    time: Option<u64>,

    #[serde(rename = "VtcId")]
    vtc_id: Option<u64>,
}

/// Live player provider based on the ETS2Map tracker.
///
/// Provides ATS live player positions.
pub struct AtsMapProvider {
    client: Client,
}

impl AtsMapProvider {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Maps TruckersMP ATS server IDs to ETS2Map tracker IDs.
    fn tracker_server_id(server_id: u64) -> u64 {
        match server_id {
            // ATS Simulation
            9 => 8,

            // ATS [US] Simulation
            11 => 10,

            // ATS [US] Arcade
            38 => 45,

            other => other,
        }
    }

    async fn get_live_players_for_server(
        &self,
        server_id: u64,
        server_name: &str,
    ) -> Result<Vec<LivePlayer>, String> {
        let tracker_server_id =
            Self::tracker_server_id(server_id);

        println!(
            "RoadWatch ATSMap provider: TMP server ID {} -> tracker ID {}.",
            server_id,
            tracker_server_id
        );

        let response = self
            .client
            .get(LIVE_AREA_URL)
            .query(&[
                ("x1", GLOBAL_ATS_AREA.x1),
                ("y1", GLOBAL_ATS_AREA.y1),
                ("x2", GLOBAL_ATS_AREA.x2),
                ("y2", GLOBAL_ATS_AREA.y2),
                ("server", tracker_server_id as i64),
            ])
            .header("User-Agent", "RoadWatch/0.1")
            .send()
            .await
            .map_err(|error| {
                format!(
                    "ATSMap provider request failed for server {server_id}: {error}"
                )
            })?;

        let status = response.status();

        if !status.is_success() {
            return Err(format!(
                "ATSMap provider returned HTTP {status} for server {server_id}"
            ));
        }

        let payload: AreaResponse = response
            .json()
            .await
            .map_err(|error| {
                format!(
                    "Could not parse ATSMap response for server {server_id}: {error}"
                )
            })?;

        if !payload.success {
            return Ok(Vec::new());
        }

        let players = payload
            .data
            .into_iter()
            .map(|player| {
                let _ = player.player_id;
                let _ = player.server_type;
                let _ = player.vtc_id;

                LivePlayer {
                    truckersmp_id: player.mp_id,
                    username: player.name,
                    status: PlayerStatus::Online,
                    game: Some(GameType::Ats),

                    // Keep the original TruckersMP server ID.
                    server_id: player
                        .server_id
                        .or(Some(server_id)),

                    server: Some(
                        server_name.to_string(),
                    ),

                    location: None,

                    position: Some(LivePosition {
                        x: player.x,
                        y: player.y,
                        z: None,
                    }),

                    heading: player.heading,
                    timestamp: player.time,
                }
            })
            .collect();

        Ok(players)
    }

    async fn fetch_live_players(
        &self,
    ) -> Result<Vec<LivePlayer>, String> {
        let server_service =
            TruckersMpServerService::new();

        let servers = server_service
            .get_ats_servers()
            .await?;

        println!(
            "RoadWatch ATSMap provider: {} ATS servers found.",
            servers.len()
        );

        let mut players_by_mp_id:
            HashMap<u64, LivePlayer> =
            HashMap::new();

        for server in servers {
            if !server.online {
                continue;
            }

            println!(
                "RoadWatch ATSMap provider: checking {} (ID {}).",
                server.name,
                server.id
            );

            match self
                .get_live_players_for_server(
                    server.id,
                    &server.name,
                )
                .await
            {
                Ok(players) => {
                    println!(
                        "RoadWatch ATSMap provider: {} players received from {}.",
                        players.len(),
                        server.name
                    );

                    for player in players {
                        players_by_mp_id.insert(
                            player.truckersmp_id,
                            player,
                        );
                    }
                }

                Err(error) => {
                    eprintln!(
                        "RoadWatch ATSMap provider: skipping {} (ID {}): {}",
                        server.name,
                        server.id,
                        error
                    );
                }
            }
        }

        let mut players: Vec<LivePlayer> =
            players_by_mp_id
                .into_values()
                .collect();

        players.sort_by_key(
            |player| player.truckersmp_id,
        );

        println!(
            "RoadWatch ATSMap provider: {} unique ATS players received.",
            players.len()
        );

        Ok(players)
    }
}

impl LivePlayerProvider for AtsMapProvider {
    fn name(&self) -> &'static str {
        "ATSMap"
    }

    fn get_live_players(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<Vec<LivePlayer>, String>,
                > + Send
                + '_,
        >,
    > {
        Box::pin(async move {
            self.fetch_live_players().await
        })
    }
}

impl Default for AtsMapProvider {
    fn default() -> Self {
        Self::new()
    }
}