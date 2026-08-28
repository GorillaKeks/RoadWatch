import { useEffect, useState } from "react";
import { ArrowLeft, Languages, Save, Truck } from "lucide-react";
import { useTranslation } from "react-i18next";

import type { AppSettings } from "../../types/settings";
import {
  getSettings,
  saveSettings,
} from "../../services/tauri/settingsService";

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const { i18n } = useTranslation();

  const [vtcId, setVtcId] = useState("");
  const [truckersmpId, setTruckersmpId] = useState("");

  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [message, setMessage] = useState("");

  useEffect(() => {
    getSettings()
      .then((settings) => {
        setVtcId(settings.vtcId);
        setTruckersmpId(settings.truckersmpId);
      })
      .catch((error) => {
        console.error("Failed to load settings:", error);
        setMessage("Einstellungen konnten nicht geladen werden.");
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  function handleVtcIdChange(value: string) {
    setVtcId(value.replace(/[^0-9]/g, ""));
    setMessage("");
  }

  function handleTruckersMpIdChange(value: string) {
    setTruckersmpId(value.replace(/[^0-9]/g, ""));
    setMessage("");
  }

  async function handleSave() {
    if (!vtcId.trim()) {
      setMessage("Bitte gib eine TruckersMP VTC-ID ein.");
      return;
    }

    if (!truckersmpId.trim()) {
      setMessage("Bitte gib deine TruckersMP-ID ein.");
      return;
    }

    setIsSaving(true);
    setMessage("");

    const settings: AppSettings = {
      vtcId,
      truckersmpId,
    };

    try {
      const saved = await saveSettings(settings);

      setVtcId(saved.vtcId);
      setTruckersmpId(saved.truckersmpId);

      setMessage("Einstellungen wurden gespeichert.");
    } catch (error) {
      console.error("Failed to save settings:", error);

      setMessage(
        typeof error === "string"
          ? error
          : "Einstellungen konnten nicht gespeichert werden.",
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
          Zurück
        </button>

        <div>
          <h2>Einstellungen</h2>
          <p>Konfiguriere deine RoadWatch-Verbindung.</p>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-header">
          <div className="settings-section-icon">
            <Truck size={20} />
          </div>

          <div>
            <h3>TruckersMP</h3>
            <p>Verbindung zu deiner VTC und deinem TruckersMP-Konto.</p>
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-vtc-id">
            TruckersMP VTC-ID
          </label>

          <input
            id="settings-vtc-id"
            type="text"
            inputMode="numeric"
            value={vtcId}
            onChange={(event) =>
              handleVtcIdChange(event.target.value)
            }
            placeholder="z. B. 7265"
            disabled={isLoading || isSaving}
          />

          <span>
            Die ID deiner TruckersMP VTC.
          </span>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-truckersmp-id">
            Meine TruckersMP-ID
          </label>

          <input
            id="settings-truckersmp-id"
            type="text"
            inputMode="numeric"
            value={truckersmpId}
            onChange={(event) =>
              handleTruckersMpIdChange(event.target.value)
            }
            placeholder="z. B. 12345678"
            disabled={isLoading || isSaving}
          />

          <span>
            Deine persönliche TruckersMP-ID. Sie wird später für
            deine eigene Live-Position und die Entfernungsberechnung
            verwendet.
          </span>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-header">
          <div className="settings-section-icon">
            <Languages size={20} />
          </div>

          <div>
            <h3>Allgemein</h3>
            <p>Allgemeine RoadWatch-Einstellungen.</p>
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-language">
            Sprache
          </label>

          <select
            id="settings-language"
            value={
              i18n.language.startsWith("de")
                ? "de"
                : "en"
            }
            onChange={(event) =>
              i18n.changeLanguage(event.target.value)
            }
            disabled={isLoading || isSaving}
          >
            <option value="de">Deutsch</option>
            <option value="en">English</option>
          </select>
        </div>
      </div>

      <div className="settings-actions">
        <button
          type="button"
          className="settings-save-button"
          onClick={handleSave}
          disabled={isLoading || isSaving}
        >
          <Save size={17} />
          {isSaving ? "Speichern..." : "Speichern"}
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