import type { Player } from "../../types/player";
import { getDistanceInKm } from "../../utils/distance";
import { PlayerCard } from "./PlayerCard";

interface PlayerListProps {
  players: Player[];
}

function sortPlayers(players: Player[]): Player[] {
  return [...players].sort((a, b) => {
    const aOnline = a.status === "online";

    const bOnline = b.status === "online";

    // Online players are always shown first.
    if (aOnline !== bOnline) {
      return aOnline ? -1 : 1;
    }

    // Sort online players with a valid distance
    // by nearest first.
    if (aOnline && bOnline) {
      const distanceA = getDistanceInKm(a.distance);

      const distanceB = getDistanceInKm(b.distance);

      if (distanceA !== null && distanceB !== null) {
        if (distanceA !== distanceB) {
          return distanceA - distanceB;
        }
      }

      // Players with a valid distance are shown
      // before players without a distance.
      if (distanceA !== null && distanceB === null) {
        return -1;
      }

      if (distanceA === null && distanceB !== null) {
        return 1;
      }
    }

    // Sort remaining players alphabetically.
    return a.username.localeCompare(b.username, undefined, {
      sensitivity: "base",
    });
  });
}

export function PlayerList({ players }: PlayerListProps) {
  if (players.length === 0) {
    return (
      <section className="player-list">
        <div className="empty-state">No players found.</div>
      </section>
    );
  }

  const sortedPlayers = sortPlayers(players);

  return (
    <section className="player-list">
      {sortedPlayers.map((player) => (
        <PlayerCard key={player.id} player={player} />
      ))}
    </section>
  );
}
