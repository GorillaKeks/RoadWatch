use crate::models::{GameType, Player, PlayerStatus};
use crate::services::settings::SettingsStore;
use crate::services::truckersmp::live::enricher::LiveEnricher;
use crate::services::truckersmp::live::mapper::apply_live_data_batch;
use crate::services::truckersmp::live::TruckersMpLiveClient;
use crate::services::truckersmp::TruckersMpClient;

/// ETS2 uses an approximate 1:19 world scale.
const ETS2_MAP_SCALE: f64 = 19.0;

/// ATS currently uses the same coordinate-to-distance scale.
///
/// Keeping this separate from ETS2 makes future adjustments easy
/// if more precise ATS map scaling data becomes available.
const ATS_MAP_SCALE: f64 = 19.0;

#[tauri::command]
pub async fn get_players() -> Result<Vec<Player>, String> {
    let settings = SettingsStore::load()?;

    if settings.vtc_id.trim().is_empty() {
        return Err(
            "Bitte zuerst eine TruckersMP VTC-ID in den Einstellungen speichern.".to_string(),
        );
    }

    if settings.truckersmp_id.trim().is_empty() {
        return Err("Bitte zuerst deine TruckersMP-ID in den Einstellungen speichern.".to_string());
    }

    let own_truckersmp_id: u64 = settings
        .truckersmp_id
        .trim()
        .parse()
        .map_err(|_| "Die gespeicherte TruckersMP-ID ist ungültig.".to_string())?;

    println!(
        "RoadWatch: own TruckersMP ID from settings: {}",
        own_truckersmp_id
    );

    let client = TruckersMpClient::new();

    let mut players = client.get_vtc_members(&settings.vtc_id).await?;

    println!("RoadWatch VTC scan: {} members loaded.", players.len());

    let live_client = TruckersMpLiveClient::new();

    let mut live_players = live_client.get_live_players().await?;

    println!(
        "RoadWatch live scan: {} live players received.",
        live_players.len()
    );

    match live_players
        .iter()
        .find(|player| player.truckersmp_id == own_truckersmp_id)
    {
        Some(player) => {
            println!(
                "RoadWatch own player FOUND in live data: {} (ID {}).",
                player.username, player.truckersmp_id
            );

            println!(
                "RoadWatch own player live server ID: {:?}.",
                player.server_id
            );

            println!("RoadWatch own player position: {:?}.", player.position);
        }

        None => {
            println!(
                "RoadWatch own player NOT FOUND in live data for ID {}.",
                own_truckersmp_id
            );
        }
    }

    let mut enricher = LiveEnricher::new();

    enricher.refresh_servers().await?;

    enricher.enrich_batch(&mut live_players);

    println!(
        "RoadWatch live enrichment complete: {} live players processed.",
        live_players.len()
    );

    let online_count = apply_live_data_batch(&mut players, &live_players);

    println!(
        "RoadWatch matching complete: {} of {} VTC members are online.",
        online_count,
        players.len()
    );

    calculate_player_distances(&mut players, &live_players, own_truckersmp_id);

    Ok(players)
}

fn calculate_player_distances(
    players: &mut [Player],
    live_players: &[crate::models::LivePlayer],
    own_truckersmp_id: u64,
) {
    let own_live_player = live_players
        .iter()
        .find(|player| player.truckersmp_id == own_truckersmp_id);

    let Some(own_live_player) = own_live_player else {
        println!(
            "RoadWatch distance calculation: own TruckersMP player {} is not online or was not found in live data.",
            own_truckersmp_id
        );

        return;
    };

    let Some(own_position) = own_live_player.position.as_ref() else {
        println!(
            "RoadWatch distance calculation: own TruckersMP player {} has no position.",
            own_truckersmp_id
        );

        return;
    };

    let Some(own_game) = own_live_player.game.as_ref() else {
        println!(
            "RoadWatch distance calculation: own TruckersMP player {} has no game type.",
            own_truckersmp_id
        );

        return;
    };

    println!(
        "RoadWatch distance calculation: own player {} found at X={} Y={}, game={:?}.",
        own_live_player.username, own_position.x, own_position.y, own_game
    );

    let mut calculated_count = 0usize;
    let mut different_game_count = 0usize;

    for player in players {
        player.distance = None;

        // The own player never gets a distance to themselves.
        if player.truckersmp_id == own_truckersmp_id {
            continue;
        }

        if player.status != PlayerStatus::Online {
            continue;
        }

        let Some(player_position) = player.live_position.as_ref() else {
            continue;
        };

        let Some(player_game) = player.game.as_ref() else {
            continue;
        };

        // ETS2 and ATS coordinates belong to completely separate maps.
        if !same_game(own_game, player_game) {
            different_game_count += 1;
            continue;
        }

        let distance_km = calculate_distance_km(
            own_position.x,
            own_position.y,
            player_position.x,
            player_position.y,
            own_game,
        );

        player.distance = Some(format_distance(distance_km));

        calculated_count += 1;

        println!(
            "RoadWatch distance: {} [{}] -> {:.2} km",
            player.username,
            game_name(own_game),
            distance_km
        );
    }

    println!(
        "RoadWatch distance calculation complete: {} distances calculated, {} online players skipped because they are in another game.",
        calculated_count,
        different_game_count
    );
}

fn same_game(first: &GameType, second: &GameType) -> bool {
    matches!(
        (first, second),
        (GameType::Ets2, GameType::Ets2) | (GameType::Ats, GameType::Ats)
    )
}

fn game_name(game: &GameType) -> &'static str {
    match game {
        GameType::Ets2 => "ETS2",
        GameType::Ats => "ATS",
    }
}

/// Calculates straight-line distance between two game-world
/// positions and converts it to real-world kilometres.
fn calculate_distance_km(x1: f64, y1: f64, x2: f64, y2: f64, game: &GameType) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let game_distance_m = (dx * dx + dy * dy).sqrt();

    let map_scale = match game {
        GameType::Ets2 => ETS2_MAP_SCALE,
        GameType::Ats => ATS_MAP_SCALE,
    };

    let real_distance_m = game_distance_m * map_scale;

    real_distance_m / 1000.0
}

fn format_distance(distance_km: f64) -> String {
    if distance_km < 1.0 {
        format!("{:.0} m", distance_km * 1000.0)
    } else if distance_km < 10.0 {
        format!("{:.1} km", distance_km)
    } else {
        format!("{:.0} km", distance_km)
    }
}
