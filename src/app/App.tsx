import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { useTranslation } from "react-i18next";
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

function App() {
  const { t } = useTranslation();

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

  const getLastUpdatedText = useCallback(
    (
      updated: Date | null,
    ): string => {
      if (!updated) {
        return t("lastUpdated.notUpdated");
      }

      const secondsAgo = Math.max(
        0,
        Math.floor(
          (Date.now() - updated.getTime()) /
            1_000,
        ),
      );

      if (secondsAgo < 5) {
        return t("lastUpdated.justNow");
      }

      if (secondsAgo < 60) {
        return t("lastUpdated.seconds", {
          count: secondsAgo,
        });
      }

      const minutesAgo = Math.floor(
        secondsAgo / 60,
      );

      if (minutesAgo === 1) {
        return t("lastUpdated.oneMinute");
      }

      if (minutesAgo < 60) {
        return t("lastUpdated.minutes", {
          count: minutesAgo,
        });
      }

      const hoursAgo = Math.floor(
        minutesAgo / 60,
      );

      if (hoursAgo === 1) {
        return t("lastUpdated.oneHour");
      }

      return t("lastUpdated.hours", {
        count: hoursAgo,
      });
    },
    [t],
  );

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
          t("dashboard.playerUpdateFailed"),
        );
      } finally {
        setIsLoadingPlayers(false);
        setIsRefreshingPlayers(false);

        isRequestRunningRef.current = false;
      }
    },
    [t],
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
            {t("dashboard.backend")}:{" "}
            {backendStatus}
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
                ? t("dashboard.refreshing")
                : t("dashboard.live")}
            </span>

            <span className="live-status-time">
              {" · "}
              {isRefreshingPlayers
                ? t(
                    "dashboard.loadingPlayers",
                  )
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
              title={t("dashboard.refreshNow")}
              aria-label={t(
                "dashboard.refreshNow",
              )}
            >
              <RefreshCw
                size={16}
                className={
                  isRefreshingPlayers
                    ? "refresh-icon-spinning"
                    : ""
                }
              />

              {t("dashboard.refresh")}
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
          title={t("dashboard.settings")}
          aria-label={t(
            "dashboard.settings",
          )}
        >
          <Settings size={20} />
        </button>
      </div>

      {currentPlayer && (
        <section className="current-player-section">
          <h2>
            {t("dashboard.currentPlayer")}
          </h2>

          <PlayerCard
            player={currentPlayer}
          />
        </section>
      )}

      <section className="vtc-members-section">
        <h2>
          {t("dashboard.members")}
        </h2>

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
              {t(
                "dashboard.loadingPlayers",
              )}
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