import { useEffect, useState } from "react";
import {
  ArrowLeft,
  Save,
  Truck,
} from "lucide-react";

import type {
  AppSettings,
} from "../../types/settings";

import {
  getSettings,
  saveSettings,
} from "../../services/tauri/settingsService";

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({
  onBack,
}: SettingsPageProps) {
  const [vtcId, setVtcId] =
    useState("");

  const [
    truckersmpId,
    setTruckersmpId,
  ] = useState("");

  const [isLoading, setIsLoading] =
    useState(true);

  const [isSaving, setIsSaving] =
    useState(false);

  const [message, setMessage] =
    useState("");

  useEffect(() => {
    getSettings()
      .then((settings) => {
        setVtcId(settings.vtcId);
        setTruckersmpId(
          settings.truckersmpId,
        );
      })
      .catch((error) => {
        console.error(
          "Failed to load settings:",
          error,
        );

        setMessage(
          "Settings could not be loaded.",
        );
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  function handleVtcIdChange(
    value: string,
  ) {
    setVtcId(
      value.replace(/[^0-9]/g, ""),
    );

    setMessage("");
  }

  function handleTruckersMpIdChange(
    value: string,
  ) {
    setTruckersmpId(
      value.replace(/[^0-9]/g, ""),
    );

    setMessage("");
  }

  async function handleSave() {
    if (!vtcId.trim()) {
      setMessage(
        "Please enter a TruckersMP VTC ID.",
      );
      return;
    }

    if (!truckersmpId.trim()) {
      setMessage(
        "Please enter your TruckersMP ID.",
      );
      return;
    }

    setIsSaving(true);
    setMessage("");

    const settings: AppSettings = {
      vtcId,
      truckersmpId,
    };

    try {
      const saved =
        await saveSettings(settings);

      setVtcId(saved.vtcId);

      setTruckersmpId(
        saved.truckersmpId,
      );

      setMessage(
        "Settings have been saved.",
      );
    } catch (error) {
      console.error(
        "Failed to save settings:",
        error,
      );

      setMessage(
        typeof error === "string"
          ? error
          : "Settings could not be saved.",
      );
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <section className="settings-page">
      <div className="settings-page-header">
        <button
          type="button"
          className="settings-back-button"
          onClick={onBack}
        >
          <ArrowLeft size={18} />
          Back
        </button>

        <div>
          <h2>Settings</h2>

          <p>
            Configure your RoadWatch connection.
          </p>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-header">
          <div className="settings-section-icon">
            <Truck size={20} />
          </div>

          <div>
            <h3>TruckersMP</h3>

            <p>
              Connection to your VTC and
              TruckersMP account.
            </p>
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-vtc-id">
            TruckersMP VTC ID
          </label>

          <input
            id="settings-vtc-id"
            type="text"
            inputMode="numeric"
            value={vtcId}
            onChange={(event) =>
              handleVtcIdChange(
                event.target.value,
              )
            }
            placeholder="e.g. 12345"
            disabled={
              isLoading || isSaving
            }
          />

          <span>
            The ID of your TruckersMP VTC.
          </span>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-truckersmp-id">
            My TruckersMP ID
          </label>

          <input
            id="settings-truckersmp-id"
            type="text"
            inputMode="numeric"
            value={truckersmpId}
            onChange={(event) =>
              handleTruckersMpIdChange(
                event.target.value,
              )
            }
            placeholder="e.g. 12345678"
            disabled={
              isLoading || isSaving
            }
          />

          <span>
            Your personal TruckersMP ID.
            It is used for your own live
            position and distance calculations.
          </span>
        </div>
      </div>

      <div className="settings-actions">
        <button
          type="button"
          className="settings-save-button"
          onClick={handleSave}
          disabled={
            isLoading || isSaving
          }
        >
          <Save size={17} />

          {isSaving
            ? "Saving..."
            : "Save"}
        </button>

        {message && (
          <p className="settings-message">
            {message}
          </p>
        )}
      </div>
    </section>
  );
}
