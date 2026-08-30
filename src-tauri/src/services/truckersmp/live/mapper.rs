use std::collections::HashMap;

use crate::models::{LivePlayer, Player};

pub fn apply_live_data(player: &mut Player, live_players: &[LivePlayer]) {
    let Some(live_player) = live_players
        .iter()
        .find(|item| item.truckersmp_id == player.truckersmp_id)
    else {
        return;
    };

    player.status = live_player.status.clone();
    player.game = live_player.game.clone();
    player.server = live_player.server.clone();
    player.location = live_player.location.clone();
    player.live_position = live_player.position.clone();
}

pub fn apply_live_data_batch(players: &mut [Player], live_players: &[LivePlayer]) -> usize {
    let live_by_mp_id: HashMap<u64, &LivePlayer> = live_players
        .iter()
        .map(|live_player| (live_player.truckersmp_id, live_player))
        .collect();

    let mut online_count = 0;

    for player in players {
        let Some(live_player) = live_by_mp_id.get(&player.truckersmp_id) else {
            continue;
        };

        player.status = live_player.status.clone();
        player.game = live_player.game.clone();
        player.server = live_player.server.clone();
        player.location = live_player.location.clone();
        player.live_position = live_player.position.clone();

        online_count += 1;
    }

    online_count
}
