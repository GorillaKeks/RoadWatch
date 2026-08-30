import type { Player } from "../../types/player";
import { PlayerCard } from "./PlayerCard";

interface PlayerListProps {
  players: Player[];
}

function getDistanceInKm(
  distance: string | null | undefined,
): number {
  if (!distance) {
    return Number.POSITIVE_INFINITY;
  }

  const normalized = distance
    .replace(",", ".")
    .trim()
    .toLowerCase();

  const value =
    Number.parseFloat(normalized);

  if (!Number.isFinite(value)) {
    return Number.POSITIVE_INFINITY;
  }

  if (normalized.endsWith("m")) {
    return value / 1000;
  }

  return value;
}

function sortPlayers(
  players: Player[],
): Player[] {
  return [...players].sort((a, b) => {
    const aOnline =
      a.status === "online";

    const bOnline =
      b.status === "online";

    // Always show online players first.
    if (aOnline !== bOnline) {
      return aOnline ? -1 : 1;
    }

    // Sort online players by distance.
    if (aOnline && bOnline) {
      const distanceA =
        getDistanceInKm(a.distance);

      const distanceB =
        getDistanceInKm(b.distance);

      if (distanceA !== distanceB) {
        return distanceA - distanceB;
      }
    }

    // Sort equal distances and offline players
    // alphabetically by username.
    return a.username.localeCompare(
      b.username,
      undefined,
      { sensitivity: "base" },
    );
  });
}

export function PlayerList({
  players,
}: PlayerListProps) {
  if (players.length === 0) {
    return (
      <section className="player-list">
        <div className="empty-state">
          No players found.
        </div>
      </section>
    );
  }

  const sortedPlayers =
    sortPlayers(players);

  return (
    <section className="player-list">
      {sortedPlayers.map((player) => (
        <PlayerCard
          key={player.id}
          player={player}
        />
      ))}
    </section>
  );
}
