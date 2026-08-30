use crate::models::{Player, PlayerStatus};

use super::api_models::VtcMember;

pub fn map_vtc_member(member: VtcMember) -> Player {
    let role = member
        .role
        .or(member.rank_name)
        .or(member.rank)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "VTC Mitglied".to_string());

    Player {
        id: member.id,

        truckersmp_id: member.user_id,

        username: member.username,

        role,

        membership: "VTC Mitglied".to_string(),

        avatar_url: member.avatar,

        status: PlayerStatus::Offline,

        game: None,
        server: None,

        location: None,

        live_position: None,

        distance: None,
    }
}
