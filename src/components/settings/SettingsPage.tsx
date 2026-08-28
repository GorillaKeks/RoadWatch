import { useEffect, useState } from "react";
import {
  ArrowLeft,
  Languages,
  Save,
  Truck,
} from "lucide-react";
import { useTranslation } from "react-i18next";

import type { AppSettings } from "../../types/settings";
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
  const { t, i18n } = useTranslation();

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
          t("settings.messages.loadError"),
        );
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, [t]);

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
        t("settings.messages.missingVtcId"),
      );

      return;
    }

    if (!truckersmpId.trim()) {
      setMessage(
        t(
          "settings.messages.missingTruckersmpId",
        ),
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
        t("settings.messages.saved"),
      );
    } catch (error) {
      console.error(
        "Failed to save settings:",
        error,
      );

      setMessage(
        typeof error === "string"
          ? error
          : t(
              "settings.messages.saveError",
            ),
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

          {t("settings.back")}
        </button>

        <div>
          <h2>
            
          </h2>

          <p>
            {t("settings.subtitle")}
          </p>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-header">
          <div className="settings-section-icon">
            <Truck size={20} />
          </div>

          <div>
            <h3>
              {t(
                "settings.truckersmp.title",
              )}
            </h3>

            <p>
              {t(
                "settings.truckersmp.description",
              )}
            </p>
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-vtc-id">
            {t(
              "settings.truckersmp.vtcId",
            )}
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
            placeholder={t(
              "settings.truckersmp.vtcIdPlaceholder",
            )}
            disabled={
              isLoading || isSaving
            }
          />

          <span>
            {t(
              "settings.truckersmp.vtcIdHelp",
            )}
          </span>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-truckersmp-id">
            {t(
              "settings.truckersmp.myTruckersmpId",
            )}
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
            placeholder={t(
              "settings.truckersmp.truckersmpIdPlaceholder",
            )}
            disabled={
              isLoading || isSaving
            }
          />

          <span>
            {t(
              "settings.truckersmp.truckersmpIdHelp",
            )}
          </span>
        </div>
      </div>

      <div className="settings-section">
        <div className="settings-section-header">
          <div className="settings-section-icon">
            <Languages size={20} />
          </div>

          <div>
            <h3>
              {t(
                "settings.general.title",
              )}
            </h3>

            <p>
              {t(
                "settings.general.description",
              )}
            </p>
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="settings-language">
            {t(
              "settings.general.language",
            )}
          </label>

          <select
            id="settings-language"
            value={
              i18n.language.startsWith("de")
                ? "de"
                : "en"
            }
            onChange={(event) =>
              i18n.changeLanguage(
                event.target.value,
              )
            }
            disabled={
              isLoading || isSaving
            }
          >
            <option value="de">
              {t("language.de")}
            </option>

            <option value="en">
              {t("language.en")}
            </option>
          </select>
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
            ? t("settings.saving")
            : t("settings.save")}
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