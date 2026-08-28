use std::collections::HashMap;

use crate::models::LivePlayer;
use crate::services::location::{Game, LocationService};
use crate::services::truckersmp::server::TruckersMpServerService;

pub struct LiveEnricher {
    location_service: LocationService,
    servers: HashMap<u64, String>,
}

impl LiveEnricher {
    pub fn new() -> Self {
        Self {
            location_service: LocationService::new(),
            servers: HashMap::new(),
        }
    }

    /// Updates the local TruckersMP server registry.
    ///
    /// Both ETS2 and ATS servers are loaded so live players from
    /// either game can be enriched with their server names.
    pub async fn refresh_servers(&mut self) -> Result<(), String> {
        let service = TruckersMpServerService::new();

        let ets2_servers = service.get_ets2_servers().await?;
        let ats_servers = service.get_ats_servers().await?;

        self.servers.clear();

        for server in ets2_servers {
            println!(
                "RoadWatch server registry: ETS2 server {} (ID {}).",
                server.name,
                server.id
            );

            self.servers.insert(
                server.id,
                server.name,
            );
        }

        for server in ats_servers {
            println!(
                "RoadWatch server registry: ATS server {} (ID {}).",
                server.name,
                server.id
            );

            self.servers.insert(
                server.id,
                server.name,
            );
        }

        println!(
            "RoadWatch server registry: {} servers loaded.",
            self.servers.len()
        );

        Ok(())
    }

    /// Enriches a live player with server name and location.
    pub fn enrich(&self, player: &mut LivePlayer) {
        if let Some(server_id) = player.server_id {
            if let Some(server_name) =
                self.servers.get(&server_id)
            {
                player.server =
                    Some(server_name.clone());
            }
        }

        let Some(position) = &player.position else {
            return;
        };

        let Some(game) = &player.game else {
            return;
        };

        let location_game = match game {
            crate::models::GameType::Ets2 => Game::Ets2,

            crate::models::GameType::Ats => Game::Ats,
        };

        let location = self.location_service.resolve(
            location_game,
            position.x,
            position.y,
        );

        player.location = Some(location);
    }

    /// Enriches all live players.
    pub fn enrich_batch(
        &self,
        players: &mut [LivePlayer],
    ) {
        for player in players {
            self.enrich(player);
        }
    }
}

impl Default for LiveEnricher {
    fn default() -> Self {
        Self::new()
    }
}