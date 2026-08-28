import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  RefreshCw,
  Settings,
} from "lucide-react";

import { AppHeader } from "../components/layout/AppHeader";
import { PlayerList } from "../components/players/PlayerList";
import { PlayerSearch } from "../components/players/PlayerSearch";
import {
  StatusFilter,
  type FilterStatus,
} from "../components/players/StatusFilter";
import { PlayerCard } from "../components/players/PlayerCard";
import { SettingsPage } from "../components/settings/SettingsPage";

import type { Player } from "../types/player";
import { getBackendStatus } from "../services/tauri/appService";
import { getPlayers } from "../services/tauri/playerService";
import { getSettings } from "../services/tauri/settingsService";

const PLAYER_REFRESH_INTERVAL_MS = 60_000;
const TIME_UPDATE_INTERVAL_MS = 1_000;

function getLastUpdatedText(
  lastUpdated: Date | null,
): string {
  if (!lastUpdated) {
    return "Noch nicht aktualisiert";
  }

  const secondsAgo = Math.max(
    0,
    Math.floor(
      (Date.now() - lastUpdated.getTime()) / 1_000,
    ),
  );

  if (secondsAgo < 5) {
    return "gerade eben aktualisiert";
  }

  if (secondsAgo < 60) {
    return `vor ${secondsAgo} Sekunden aktualisiert`;
  }

  const minutesAgo = Math.floor(
    secondsAgo / 60,
  );

  if (minutesAgo === 1) {
    return "vor 1 Minute aktualisiert";
  }

  if (minutesAgo < 60) {
    return `vor ${minutesAgo} Minuten aktualisiert`;
  }

  const hoursAgo = Math.floor(
    minutesAgo / 60,
  );

  if (hoursAgo === 1) {
    return "vor 1 Stunde aktualisiert";
  }

  return `vor ${hoursAgo} Stunden aktualisiert`;
}

function App() {
  const [searchTerm, setSearchTerm] =
    useState("");

  const [statusFilter, setStatusFilter] =
    useState<FilterStatus>("online");

  const [backendStatus, setBackendStatus] =
    useState("Connecting...");

  const [players, setPlayers] =
    useState<Player[]>([]);

  const [currentPlayer, setCurrentPlayer] =
    useState<Player | null>(null);

  const [isLoadingPlayers, setIsLoadingPlayers] =
    useState(true);

  const [
    isRefreshingPlayers,
    setIsRefreshingPlayers,
  ] = useState(false);

  const [playerError, setPlayerError] =
    useState<string | null>(null);

  const [lastUpdated, setLastUpdated] =
    useState<Date | null>(null);

  const [, setTimeTick] =
    useState(0);

  const [showSettings, setShowSettings] =
    useState(false);

  const isRequestRunningRef =
    useRef(false);

  useEffect(() => {
    getBackendStatus()
      .then(setBackendStatus)
      .catch(() =>
        setBackendStatus("Backend unavailable"),
      );
  }, []);

  const loadPlayers = useCallback(
    async (initialLoad = false) => {
      if (isRequestRunningRef.current) {
        return;
      }

      isRequestRunningRef.current = true;

      try {
        if (initialLoad) {
          setIsLoadingPlayers(true);
        } else {
          setIsRefreshingPlayers(true);
        }

        setPlayerError(null);

        const backendPlayers =
          await getPlayers();

        setPlayers(backendPlayers);

        const settings =
          await getSettings();

        const ownTruckersMpId =
          Number(settings.truckersmpId);

        if (ownTruckersMpId > 0) {
          const ownPlayer =
            backendPlayers.find(
              (player) =>
                player.truckersmpId ===
                ownTruckersMpId,
            ) ?? null;

          setCurrentPlayer(ownPlayer);
        } else {
          setCurrentPlayer(null);
        }

        setLastUpdated(new Date());
      } catch (error) {
        console.error(
          "Failed to load players:",
          error,
        );

        setPlayerError(
          "Spielerdaten konnten nicht aktualisiert werden.",
        );
      } finally {
        setIsLoadingPlayers(false);
        setIsRefreshingPlayers(false);

        isRequestRunningRef.current = false;
      }
    },
    [],
  );

  useEffect(() => {
    if (showSettings) {
      return;
    }

    void loadPlayers(true);

    const intervalId =
      window.setInterval(() => {
        void loadPlayers(false);
      }, PLAYER_REFRESH_INTERVAL_MS);

    return () => {
      window.clearInterval(intervalId);
    };
  }, [
    loadPlayers,
    showSettings,
  ]);

  useEffect(() => {
    const intervalId =
      window.setInterval(() => {
        setTimeTick(
          (value) => value + 1,
        );
      }, TIME_UPDATE_INTERVAL_MS);

    return () => {
      window.clearInterval(intervalId);
    };
  }, []);

  const filteredPlayers = useMemo(() => {
    return players.filter((player) => {
      const query =
        searchTerm.toLowerCase();

      const matchesSearch =
        player.username
          .toLowerCase()
          .includes(query) ||
        player.location?.city
          ?.toLowerCase()
          .includes(query) ||
        player.location?.country
          ?.toLowerCase()
          .includes(query) ||
        player.location?.region
          ?.toLowerCase()
          .includes(query);

      const matchesStatus =
        statusFilter === "all" ||
        player.status === statusFilter;

      return (
        matchesSearch &&
        matchesStatus
      );
    });
  }, [
    players,
    searchTerm,
    statusFilter,
  ]);

  const onlineCount = players.filter(
    (player) =>
      player.status === "online",
  ).length;

  const lastUpdatedText =
    getLastUpdatedText(lastUpdated);

  if (showSettings) {
    return (
      <main className="app-shell">
        <SettingsPage
          onBack={() =>
            setShowSettings(false)
          }
        />
      </main>
    );
  }

  return (
    <main className="app-shell">
      <div className="top-bar">
        <div>
          <AppHeader
            memberCount={players.length}
            onlineCount={onlineCount}
          />

          <div className="backend-status">
            Rust: {backendStatus}
          </div>

          <div className="live-status">
            <span
              className={`live-status-dot ${
                isRefreshingPlayers
                  ? "refreshing"
                  : "live"
              }`}
            />

            <span className="live-status-label">
              {isRefreshingPlayers
                ? "Aktualisiere"
                : "Live"}
            </span>

            <span className="live-status-time">
              ·{" "}
              {isRefreshingPlayers
                ? "Daten werden aktualisiert..."
                : lastUpdatedText}
            </span>

            <button
              type="button"
              className="refresh-button"
              onClick={() =>
                void loadPlayers(false)
              }
              disabled={
                isLoadingPlayers ||
                isRefreshingPlayers
              }
              title="Jetzt aktualisieren"
              aria-label="Jetzt aktualisieren"
            >
              <RefreshCw
                size={16}
                className={
                  isRefreshingPlayers
                    ? "refresh-icon-spinning"
                    : ""
                }
              />

              Aktualisieren
            </button>
          </div>

          {playerError &&
            players.length > 0 && (
              <div className="refresh-error">
                {playerError}
              </div>
            )}
        </div>

        <button
          type="button"
          className="settings-button"
          onClick={() =>
            setShowSettings(true)
          }
          title="Einstellungen"
          aria-label="Einstellungen"
        >
          <Settings size={20} />
        </button>
      </div>

      {currentPlayer && (
        <section className="current-player-section">
          <br />

          <PlayerCard
            player={currentPlayer}
          />
        </section>
      )}

      <section className="vtc-members-section">
        <h2>VTC Mitglieder</h2>

        <section className="controls">
          <PlayerSearch
            value={searchTerm}
            onChange={setSearchTerm}
          />

          <StatusFilter
            value={statusFilter}
            onChange={setStatusFilter}
          />
        </section>

        {isLoadingPlayers &&
          players.length === 0 && (
            <div className="empty-state">
              Spielerdaten werden geladen...
            </div>
          )}

        {playerError &&
          players.length === 0 && (
            <div className="empty-state">
              {playerError}
            </div>
          )}

        {!isLoadingPlayers &&
          players.length > 0 && (
            <PlayerList
              players={filteredPlayers.filter(
                (player) =>
                  player.truckersmpId !==
                  currentPlayer?.truckersmpId,
              )}
            />
          )}

        {!isLoadingPlayers &&
          !playerError &&
          players.length === 0 && (
            <PlayerList
              players={filteredPlayers}
            />
          )}
      </section>
    </main>
  );
}

export default App;