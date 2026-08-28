import { useTranslation } from "react-i18next";

interface AppHeaderProps {
  memberCount: number;
  onlineCount: number;
}

export function AppHeader({
  memberCount,
  onlineCount,
}: AppHeaderProps) {
  const { t } = useTranslation();

  return (
    <header className="app-header">
      <h1>{t("app.name")}</h1>

      <div className="statistics">
        <span>
          {t("dashboard.members")}:{" "}
          <strong>{memberCount}</strong>
        </span>

        <span className="separator">•</span>

        <span>
          {t("dashboard.online")}:{" "}
          <strong className="online-text">
            {onlineCount}
          </strong>
        </span>
      </div>
    </header>
  );
}