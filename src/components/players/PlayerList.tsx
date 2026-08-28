import { useTranslation } from "react-i18next";
import type { Player } from "../../types/player";
import { PlayerCard } from "./PlayerCard";

interface PlayerListProps {
  players: Player[];
}

function getDistanceInKm(distance: string | null | undefined): number {
  if (!distance) {
    return Number.POSITIVE_INFINITY;
  }

  const normalized = distance
    .replace(",", ".")
    .trim()
    .toLowerCase();

  const value = Number.parseFloat(normalized);

  if (!Number.isFinite(value)) {
    return Number.POSITIVE_INFINITY;
  }

  if (normalized.endsWith("m")) {
    return value / 1000;
  }

  return value;
}

function sortPlayers(players: Player[]): Player[] {
  return [...players].sort((a, b) => {
    const aOnline = a.status === "online";
    const bOnline = b.status === "online";

    // Online-Spieler immer vor Offline-Spielern.
    if (aOnline !== bOnline) {
      return aOnline ? -1 : 1;
    }

    // Bei Online-Spielern nach Entfernung sortieren.
    if (aOnline && bOnline) {
      const distanceA = getDistanceInKm(a.distance);
      const distanceB = getDistanceInKm(b.distance);

      if (distanceA !== distanceB) {
        return distanceA - distanceB;
      }
    }

    // Bei gleicher Entfernung bzw. Offline:
    // alphabetisch nach Benutzername.
    return a.username.localeCompare(
      b.username,
      undefined,
      { sensitivity: "base" },
    );
  });
}

export function PlayerList({ players }: PlayerListProps) {
  const { t } = useTranslation();

  if (players.length === 0) {
    return (
      <section className="player-list">
        <div className="empty-state">
          {t("players.empty")}
        </div>
      </section>
    );
  }

  const sortedPlayers = sortPlayers(players);

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