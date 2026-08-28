use std::collections::HashMap;

use crate::models::LivePlayer;

use super::{
    AtsMapProvider,
    Ets2MapProvider,
    LivePlayerProvider,
};

/// Coordinates all configured live player providers.
///
/// The client itself does not depend on a specific live data source.
/// Providers can support ETS2, ATS or both games.
pub struct TruckersMpLiveClient {
    providers: Vec<Box<dyn LivePlayerProvider>>,
}

impl TruckersMpLiveClient {
    pub fn new() -> Self {
        let providers: Vec<Box<dyn LivePlayerProvider>> = vec![
            Box::new(Ets2MapProvider::new()),
            Box::new(AtsMapProvider::new()),
        ];

        Self {
            providers,
        }
    }

    /// Collects live players from all configured providers.
    ///
    /// Players with the same TruckersMP ID are merged so that
    /// duplicate results from multiple providers are avoided.
    ///
    /// If one provider fails, the remaining providers are still queried.
    pub async fn get_live_players(
        &self,
    ) -> Result<Vec<LivePlayer>, String> {
        let mut players_by_mp_id:
            HashMap<u64, LivePlayer> = HashMap::new();

        let mut successful_providers = 0usize;
        let mut errors: Vec<String> = Vec::new();

        for provider in &self.providers {
            println!(
                "RoadWatch live client: querying provider {}.",
                provider.name()
            );

            match provider.get_live_players().await {
                Ok(players) => {
                    println!(
                        "RoadWatch live client: provider {} returned {} players.",
                        provider.name(),
                        players.len()
                    );

                    for player in players {
                        players_by_mp_id.insert(
                            player.truckersmp_id,
                            player,
                        );
                    }

                    successful_providers += 1;
                }

                Err(error) => {
                    eprintln!(
                        "RoadWatch live client: provider {} failed: {}",
                        provider.name(),
                        error
                    );

                    errors.push(format!(
                        "{}: {}",
                        provider.name(),
                        error
                    ));
                }
            }
        }

        if successful_providers == 0 {
            if errors.is_empty() {
                return Err(
                    "No live player providers are configured."
                        .to_string(),
                );
            }

            return Err(format!(
                "All live player providers failed: {}",
                errors.join(" | ")
            ));
        }

        let mut players: Vec<LivePlayer> =
            players_by_mp_id
                .into_values()
                .collect();

        // Keep the result order stable and predictable.
        players.sort_by_key(
            |player| player.truckersmp_id,
        );

        println!(
            "RoadWatch live client: {} unique live players received from all providers.",
            players.len()
        );

        Ok(players)
    }
}

impl Default for TruckersMpLiveClient {
    fn default() -> Self {
        Self::new()
    }
}