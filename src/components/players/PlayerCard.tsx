import { MapPin } from "lucide-react";
import type { Player } from "../../types/player";

interface PlayerCardProps {
  player: Player;
}

export function PlayerCard({
  player,
}: PlayerCardProps) {
  const initial = player.username
    .replace(/[^a-zA-Z0-9]/g, "")
    .charAt(0)
    .toUpperCase();

  const gameLabel =
    player.game === "ets2"
      ? "ETS2"
      : player.game === "ats"
        ? "ATS"
        : undefined;

  const statusLabel =
    player.status === "online"
      ? "Online"
      : "Offline";

  return (
    <article className="player-card">
      <div className="status-column">
        <span
          className={`status-dot ${player.status}`}
          title={statusLabel}
          aria-label={statusLabel}
        />
      </div>

      <div className="player-avatar">
        {player.avatarUrl ? (
          <img
            src={player.avatarUrl}
            alt={`${player.username} avatar`}
          />
        ) : (
          <span>{initial}</span>
        )}
      </div>

      <div className="player-main">
        <div className="player-info">
          <h2>{player.username}</h2>

          {player.role && (
            <p className="player-role">
              {player.role}
            </p>
          )}

          {player.status === "online" && (
            <div className="player-meta">
              {gameLabel && (
                <span>{gameLabel}</span>
              )}

              {player.server && (
                <span>{player.server}</span>
              )}
            </div>
          )}
        </div>

        {player.location && (
          <div className="player-location">
            <MapPin
              size={19}
              strokeWidth={2}
            />

            <div>
              <strong>
                {player.location.city}
              </strong>

              <span>
                {player.location.region
                  ? `${player.location.region}, `
                  : ""}
                {player.location.country}
              </span>
            </div>
          </div>
        )}
      </div>

      <div className="distance-column">
        {player.distance ?? "—"}
      </div>
    </article>
  );
}
