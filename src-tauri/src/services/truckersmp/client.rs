use reqwest::Client;

use crate::models::Player;

use super::api_models::{ApiResponse, VtcMembersResponse};
use super::mapper::map_vtc_member;

const TRUCKERSMP_API_URL: &str = "https://api.truckersmp.com/v2";

pub struct TruckersMpClient {
    client: Client,
}

impl TruckersMpClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create TruckersMP HTTP client"),
        }
    }

    pub async fn get_vtc_members(
        &self,
        vtc_id: &str,
    ) -> Result<Vec<Player>, String> {
        if vtc_id.trim().is_empty() {
            return Ok(Vec::new());
        }

        let url = format!(
            "{TRUCKERSMP_API_URL}/vtc/{}/members",
            vtc_id.trim()
        );

        println!("RoadWatch VTC API request: {url}");

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "RoadWatch/0.1")
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|error| {
                format!("TruckersMP VTC request failed: {error}")
            })?;

        let status = response.status();

        if !status.is_success() {
            return Err(format!(
                "TruckersMP VTC request returned HTTP {status}"
            ));
        }

        let payload: ApiResponse<VtcMembersResponse> = response
            .json()
            .await
            .map_err(|error| {
                format!(
                    "Could not parse TruckersMP VTC members response: {error}"
                )
            })?;

        if payload.error {
            return Err(
                "TruckersMP API returned an error for this VTC."
                    .to_string()
            );
        }

        println!(
            "RoadWatch VTC API: {} raw members received.",
            payload.response.members.len()
        );

        let players: Vec<Player> = payload
            .response
            .members
            .into_iter()
            .map(map_vtc_member)
            .collect();

        println!(
            "RoadWatch VTC API: {} members mapped successfully.",
            players.len()
        );

        Ok(players)
    }
}