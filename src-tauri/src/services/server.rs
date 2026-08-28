use reqwest::Client;
use serde_json::Value;

const SERVERS_URL: &str = "https://api.truckersmp.com/v2/servers";

#[derive(Debug, Clone)]
pub struct TruckersMpServer {
    pub id: u64,
    pub game: String,
    pub name: String,
    pub shortname: String,
    pub online: bool,
    pub players: u64,
    pub max_players: u64,
    pub promods: bool,
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

    pub async fn get_servers(&self) -> Result<Vec<TruckersMpServer>, String> {
        let response = self
            .client
            .get(SERVERS_URL)
            .header("User-Agent", "RoadWatch/0.1")
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|error| {
                format!("TruckersMP request failed: {error}")
            })?;

        let status = response.status();

        println!("TruckersMP HTTP status: {status}");

        if !status.is_success() {
            return Err(format!(
                "TruckersMP API returned HTTP {status}"
            ));
        }

        let body = response
            .text()
            .await
            .map_err(|error| {
                format!("Could not read TruckersMP response: {error}")
            })?;

        if body.trim().is_empty() {
            return Err(
                "TruckersMP API returned an empty response.".to_string()
            );
        }

        let root: Value = serde_json::from_str(&body)
            .map_err(|error| {
                format!(
                    "TruckersMP response is not valid JSON: {error}"
                )
            })?;

        let response_array = root
            .get("response")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                format!(
                    "TruckersMP response does not contain a valid response array:\n{root}"
                )
            })?;

        let mut servers = Vec::new();

        for item in response_array {
            let id = item
                .get("id")
                .and_then(Value::as_u64)
                .unwrap_or(0);

            let game = item
                .get("game")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();

            let name = item
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("Unknown")
                .to_string();

            let shortname = item
                .get("shortname")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();

            let online = item
                .get("online")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            let players = item
                .get("players")
                .and_then(Value::as_u64)
                .unwrap_or(0);

            let max_players = item
                .get("maxplayers")
                .and_then(Value::as_u64)
                .unwrap_or(0);

            let promods = item
                .get("promods")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            servers.push(TruckersMpServer {
                id,
                game,
                name,
                shortname,
                online,
                players,
                max_players,
                promods,
            });
        }

        Ok(servers)
    }

    pub async fn get_ets2_servers(
        &self,
    ) -> Result<Vec<TruckersMpServer>, String> {
        let servers = self.get_servers().await?;

        Ok(servers
            .into_iter()
            .filter(|server| {
                server.game.eq_ignore_ascii_case("ETS2")
            })
            .collect())
    }

    pub async fn get_server(
        &self,
        server_id: u64,
    ) -> Result<Option<TruckersMpServer>, String> {
        let servers = self.get_servers().await?;

        Ok(servers
            .into_iter()
            .find(|server| server.id == server_id))
    }
}

impl Default for TruckersMpServerService {
    fn default() -> Self {
        Self::new()
    }
}