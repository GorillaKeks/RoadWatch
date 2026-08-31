import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { Download, RefreshCw, Settings, X } from "lucide-react";

import { AppHeader } from "../components/layout/AppHeader";
import { DistanceFilter } from "../components/players/DistanceFilter";
import type { DistanceFilterValue } from "../components/players/DistanceFilter";
import { PlayerCard } from "../components/players/PlayerCard";
import { PlayerList } from "../components/players/PlayerList";
import { PlayerSearch } from "../components/players/PlayerSearch";
import {
  StatusFilter,
  type FilterStatus,
} from "../components/players/StatusFilter";
import { SettingsPage } from "../components/settings/SettingsPage";

import { getBackendStatus } from "../services/tauri/appService";
import { getPlayers } from "../services/tauri/playerService";
import { getSettings } from "../services/tauri/settingsService";

import {
  checkForUpdate,
  downloadAndInstallUpdate,
} from "../services/updater/updateService";

import type { Player } from "../types/player";
import { getDistanceInKm } from "../utils/distance";

import type { Update } from "@tauri-apps/plugin-updater";

const PLAYER_REFRESH_INTERVAL_MS = 60_000;

function App() {
  // =========================================================
  // Filters
  // =========================================================

  const [searchTerm, setSearchTerm] = useState("");

  const [statusFilter, setStatusFilter] = useState<FilterStatus>("online");

  const [distanceFilter, setDistanceFilter] =
    useState<DistanceFilterValue>("all");

  // =========================================================
  // Player State
  // =========================================================

  const [players, setPlayers] = useState<Player[]>([]);

  const [currentPlayer, setCurrentPlayer] = useState<Player | null>(null);

  const [isLoadingPlayers, setIsLoadingPlayers] = useState(true);

  const [isRefreshingPlayers, setIsRefreshingPlayers] = useState(false);

  const [playerError, setPlayerError] = useState<string | null>(null);

  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  // Used to refresh the relative time display.
  const [, setTimeTick] = useState(0);

  // Prevent overlapping player requests.
  const isRequestRunningRef = useRef(false);

  // =========================================================
  // Backend State
  // =========================================================

  const [backendStatus, setBackendStatus] = useState("Connecting...");

  // =========================================================
  // Settings
  // =========================================================

  const [showSettings, setShowSettings] = useState(false);

  // =========================================================
  // Updater
  // =========================================================

  const [availableUpdate, setAvailableUpdate] = useState<Update | null>(null);

  const [isCheckingUpdate, setIsCheckingUpdate] = useState(true);

  const [isInstallingUpdate, setIsInstallingUpdate] = useState(false);

  const [updateError, setUpdateError] = useState<string | null>(null);

  const [downloadProgress, setDownloadProgress] = useState<number | null>(null);

  // =========================================================
  // Relative Last Updated Text
  // =========================================================

  const getLastUpdatedText = useCallback((updated: Date | null): string => {
    if (!updated) {
      return "Not updated yet";
    }

    const secondsAgo = Math.max(
      0,
      Math.floor((Date.now() - updated.getTime()) / 1000),
    );

    if (secondsAgo < 5) {
      return "updated just now";
    }

    if (secondsAgo < 60) {
      return `updated ${secondsAgo} seconds ago`;
    }

    const minutesAgo = Math.floor(secondsAgo / 60);

    if (minutesAgo === 1) {
      return "updated 1 minute ago";
    }

    if (minutesAgo < 60) {
      return `updated ${minutesAgo} minutes ago`;
    }

    const hoursAgo = Math.floor(minutesAgo / 60);

    if (hoursAgo === 1) {
      return "updated 1 hour ago";
    }

    return `updated ${hoursAgo} hours ago`;
  }, []);

  // =========================================================
  // Update Check
  // =========================================================

  useEffect(() => {
    let isMounted = true;

    async function runUpdateCheck() {
      try {
        const update = await checkForUpdate();

        if (isMounted) {
          setAvailableUpdate(update);
        }
      } catch (error) {
        console.error("Failed to check for updates:", error);

        if (isMounted) {
          setUpdateError("Unable to check for updates.");
        }
      } finally {
        if (isMounted) {
          setIsCheckingUpdate(false);
        }
      }
    }

    void runUpdateCheck();

    return () => {
      isMounted = false;
    };
  }, []);

  // =========================================================
  // Install Update
  // =========================================================

  async function handleInstallUpdate() {
    if (!availableUpdate || isInstallingUpdate) {
      return;
    }

    try {
      setIsInstallingUpdate(true);
      setUpdateError(null);
      setDownloadProgress(0);

      await downloadAndInstallUpdate(availableUpdate, (downloaded, total) => {
        if (total && total > 0) {
          setDownloadProgress(Math.round((downloaded / total) * 100));
        }
      });
    } catch (error) {
      console.error("Failed to install update:", error);

      setUpdateError("The update could not be installed.");

      setIsInstallingUpdate(false);
      setDownloadProgress(null);
    }
  }

  // =========================================================
  // Backend Status
  // =========================================================

  useEffect(() => {
    getBackendStatus()
      .then(setBackendStatus)
      .catch(() => setBackendStatus("Backend unavailable"));
  }, []);

  // =========================================================
  // Load Players
  // =========================================================

  const loadPlayers = useCallback(async (initialLoad = false) => {
    if (isRequestRunningRef.current) {
      return;
    }

    try {
      isRequestRunningRef.current = true;

      if (initialLoad) {
        setIsLoadingPlayers(true);
      } else {
        setIsRefreshingPlayers(true);
      }

      setPlayerError(null);

      const [backendPlayers, currentSettings] = await Promise.all([
        getPlayers(),
        getSettings(),
      ]);

      setPlayers(backendPlayers);

      const ownTruckersMpId = Number(currentSettings.truckersmpId);

      if (ownTruckersMpId > 0) {
        const ownPlayer =
          backendPlayers.find(
            (player) => player.truckersmpId === ownTruckersMpId,
          ) ?? null;

        setCurrentPlayer(ownPlayer);
      } else {
        setCurrentPlayer(null);
      }

      setLastUpdated(new Date());
    } catch (error) {
      console.error("Failed to load players:", error);

      setPlayerError("Player data could not be updated.");
    } finally {
      setIsLoadingPlayers(false);
      setIsRefreshingPlayers(false);

      isRequestRunningRef.current = false;
    }
  }, []);

  // =========================================================
  // Initial Load + Automatic Refresh
  // =========================================================

  useEffect(() => {
    void loadPlayers(true);

    const intervalId = window.setInterval(() => {
      void loadPlayers(false);
    }, PLAYER_REFRESH_INTERVAL_MS);

    return () => {
      window.clearInterval(intervalId);
    };
  }, [loadPlayers]);

  // =========================================================
  // Refresh Relative Time
  // =========================================================

  useEffect(() => {
    const intervalId = window.setInterval(() => {
      setTimeTick((value) => value + 1);
    }, 1_000);

    return () => {
      window.clearInterval(intervalId);
    };
  }, []);

  // =========================================================
  // Filter Players
  // =========================================================

  const filteredPlayers = useMemo(() => {
    const ownTruckersMpId = currentPlayer?.truckersmpId;

    return players.filter((player) => {
      // The current player is displayed
      // separately in the My Driver section.
      if (
        ownTruckersMpId !== undefined &&
        player.truckersmpId === ownTruckersMpId
      ) {
        return false;
      }

      // -------------------------------------------------
      // Search
      // -------------------------------------------------

      const query = searchTerm.trim().toLowerCase();

      const matchesSearch =
        query.length === 0 ||
        player.username.toLowerCase().includes(query) ||
        player.location?.city?.toLowerCase().includes(query) ||
        player.location?.country?.toLowerCase().includes(query) ||
        player.location?.region?.toLowerCase().includes(query);

      // -------------------------------------------------
      // Status
      // -------------------------------------------------

      const matchesStatus =
        statusFilter === "all" || player.status === statusFilter;

      // -------------------------------------------------
      // Distance
      // -------------------------------------------------

      let matchesDistance = true;

      /*
       * Distance filters apply only to
       * online players.
       *
       * Offline players have no meaningful
       * current distance and remain visible
       * when the selected status allows them.
       */
      if (player.status === "online" && distanceFilter !== "all") {
        const distanceKm = getDistanceInKm(player.distance);

        const maximumDistance = Number(distanceFilter);

        matchesDistance = distanceKm !== null && distanceKm <= maximumDistance;
      }

      return matchesSearch && matchesStatus && matchesDistance;
    });
  }, [players, currentPlayer, searchTerm, statusFilter, distanceFilter]);

  // =========================================================
  // Statistics
  // =========================================================

  const onlineCount = players.filter(
    (player) => player.status === "online",
  ).length;

  const lastUpdatedText = getLastUpdatedText(lastUpdated);

  // =========================================================
  // Settings Page
  // =========================================================

  if (showSettings) {
    return (
      <main className="app-shell">
        <SettingsPage onBack={() => setShowSettings(false)} />
      </main>
    );
  }

  // =========================================================
  // Main Application
  // =========================================================

  return (
    <main className="app-shell">
      {/* =====================================================
          Update Banner
         ===================================================== */}

      {availableUpdate && (
        <div className="update-banner">
          <div className="update-banner-content">
            <Download size={20} />

            <div>
              <strong>
                RoadWatch v{availableUpdate.version}
                {" is available"}
              </strong>

              <p>A new version is ready to download and install.</p>

              {isInstallingUpdate && downloadProgress !== null && (
                <p>Downloading update: {downloadProgress}%</p>
              )}

              {updateError && <p className="update-error">{updateError}</p>}
            </div>
          </div>

          <div className="update-banner-actions">
            <button
              type="button"
              className="update-install-button"
              onClick={() => void handleInstallUpdate()}
              disabled={isInstallingUpdate}
            >
              {isInstallingUpdate ? "Installing..." : "Download and install"}
            </button>

            {!isInstallingUpdate && (
              <button
                type="button"
                className="update-dismiss-button"
                onClick={() => setAvailableUpdate(null)}
                title="Dismiss"
                aria-label="Dismiss update"
              >
                <X size={18} />
              </button>
            )}
          </div>
        </div>
      )}

      {!availableUpdate && !isCheckingUpdate && updateError && (
        <div className="update-check-error">{updateError}</div>
      )}

      {/* =====================================================
          Top Bar
         ===================================================== */}

      <div className="top-bar">
        <div>
          <AppHeader memberCount={players.length} onlineCount={onlineCount} />

          <div className="backend-status">Rust: {backendStatus}</div>

          <div className="live-status">
            <span
              className={`live-status-dot ${
                isRefreshingPlayers ? "refreshing" : "live"
              }`}
            />

            <span className="live-status-label">
              {isRefreshingPlayers ? "Refreshing" : "Live"}
            </span>

            <span className="live-status-time">
              {isRefreshingPlayers ? "Updating data..." : lastUpdatedText}
            </span>

            <button
              type="button"
              className="refresh-button"
              onClick={() => void loadPlayers(false)}
              disabled={isRefreshingPlayers || isRequestRunningRef.current}
              title="Refresh now"
              aria-label="Refresh now"
            >
              <RefreshCw
                size={17}
                className={
                  isRefreshingPlayers ? "refresh-icon-spinning" : undefined
                }
              />

              <span>Refresh</span>
            </button>
          </div>

          {playerError && players.length > 0 && (
            <div className="refresh-error">{playerError}</div>
          )}
        </div>

        <button
          type="button"
          className="settings-button"
          onClick={() => setShowSettings(true)}
          title="Settings"
          aria-label="Settings"
        >
          <Settings size={20} />
        </button>
      </div>

      {/* =====================================================
          My Driver
         ===================================================== */}

      {currentPlayer && (
        <section className="current-player-section">
          <h2>My Driver</h2>

          <PlayerCard player={currentPlayer} />
        </section>
      )}

      {/* =====================================================
          VTC Members
         ===================================================== */}

      <section className="vtc-members-section">
        <h2>VTC Members</h2>

        {/* Controls */}

        <section className="controls">
          <PlayerSearch value={searchTerm} onChange={setSearchTerm} />

          <StatusFilter value={statusFilter} onChange={setStatusFilter} />

          <DistanceFilter value={distanceFilter} onChange={setDistanceFilter} />
        </section>

        {/* Loading */}

        {isLoadingPlayers && players.length === 0 && (
          <div className="empty-state">Loading player data...</div>
        )}

        {/* Error */}

        {playerError && players.length === 0 && (
          <div className="empty-state">{playerError}</div>
        )}

        {/* Player List */}

        {!isLoadingPlayers && !playerError && (
          <PlayerList players={filteredPlayers} />
        )}
      </section>
    </main>
  );
}

export default App;
