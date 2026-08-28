use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

const SERVERS_URL: &str = "https://api.truckersmp.com/v2/servers";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TruckersMpServer {
    pub id: u64,
    pub game: String,
    pub name: String,
    pub shortname: String,
    pub online: bool,
    pub players: u64,
    pub queue: u64,

    #[serde(rename = "maxplayers")]
    pub max_players: u64,

    pub promods: bool,
    pub event: bool,

    #[serde(rename = "specialEvent")]
    pub special_event: bool,

    pub speedlimiter: u64,
    pub collisions: bool,
    pub carsforplayers: bool,
}

pub struct TruckersMpServerService {
    client: Client,
}

impl TruckersMpServerService {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn get_servers(
        &self,
    ) -> Result<Vec<TruckersMpServer>, String> {
        let response = self
            .client
            .get(SERVERS_URL)
            .header("User-Agent", "RoadWatch/0.1")
            .send()
            .await
            .map_err(|error| {
                format!(
                    "TruckersMP server request failed: {error}"
                )
            })?;

        let status = response.status();

        println!("TruckersMP HTTP status: {status}");

        if !status.is_success() {
            return Err(format!(
                "TruckersMP server API returned HTTP {status}"
            ));
        }

        let body = response
            .text()
            .await
            .map_err(|error| {
                format!(
                    "Could not read TruckersMP server response: {error}"
                )
            })?;

        let json: Value = serde_json::from_str(&body)
            .map_err(|error| {
                format!(
                    "Could not parse TruckersMP server JSON: {error}"
                )
            })?;

        let error_value = json
            .get("error")
            .cloned()
            .unwrap_or(Value::Bool(false));

        let api_error = match error_value {
            Value::Bool(value) => value,

            Value::String(value) => {
                value.eq_ignore_ascii_case("true")
            }

            _ => false,
        };

        if api_error {
            return Err(
                "TruckersMP server API returned an error."
                    .to_string(),
            );
        }

        let response_value = json
            .get("response")
            .cloned()
            .unwrap_or_else(|| Value::Array(Vec::new()));

        let servers: Vec<TruckersMpServer> =
            serde_json::from_value(response_value)
                .map_err(|error| {
                    format!(
                        "Could not decode TruckersMP server list: {error}"
                    )
                })?;

        println!(
            "RoadWatch TruckersMP API: {} servers received.",
            servers.len()
        );

        Ok(servers)
    }

    /// Returns regular TruckersMP servers for the specified game.
    ///
    /// Event and special-event servers are excluded.
    async fn get_regular_servers_for_game(
        &self,
        game: &str,
    ) -> Result<Vec<TruckersMpServer>, String> {
        let servers = self.get_servers().await?;

        let filtered_servers = servers
            .into_iter()
            .filter(|server| {
                server.game.eq_ignore_ascii_case(game)
                    && !server.event
                    && !server.special_event
            })
            .collect::<Vec<_>>();

        println!(
            "RoadWatch: {} regular {} servers found.",
            filtered_servers.len(),
            game
        );

        for server in &filtered_servers {
            println!(
                "RoadWatch {} server: {} (ID {}) - {}/{} players",
                game,
                server.name,
                server.id,
                server.players,
                server.max_players
            );
        }

        Ok(filtered_servers)
    }

    /// Returns only regular ETS2 TruckersMP servers.
    ///
    /// Event and special-event servers are excluded.
    pub async fn get_ets2_servers(
        &self,
    ) -> Result<Vec<TruckersMpServer>, String> {
        self.get_regular_servers_for_game("ETS2")
            .await
    }

    /// Returns only regular ATS TruckersMP servers.
    ///
    /// Event and special-event servers are excluded.
    pub async fn get_ats_servers(
        &self,
    ) -> Result<Vec<TruckersMpServer>, String> {
        self.get_regular_servers_for_game("ATS")
            .await
    }

    pub async fn get_server(
        &self,
        server_id: u64,
    ) -> Result<Option<TruckersMpServer>, String> {
        let servers = self.get_servers().await?;

        Ok(
            servers
                .into_iter()
                .find(|server| server.id == server_id),
        )
    }
}

impl Default for TruckersMpServerService {
    fn default() -> Self {
        Self::new()
    }
}