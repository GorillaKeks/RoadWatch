import { useTranslation } from "react-i18next";

export type FilterStatus = "all" | "online" | "offline";

interface StatusFilterProps {
  value: FilterStatus;
  onChange: (value: FilterStatus) => void;
}

export function StatusFilter({ value, onChange }: StatusFilterProps) {
  const { t } = useTranslation();

  return (
    <select
      className="status-filter"
      value={value}
      onChange={(event) => onChange(event.target.value as FilterStatus)}
    >
      <option value="all">{t("dashboard.all")}</option>
      <option value="online">{t("dashboard.online")}</option>
      <option value="offline">{t("dashboard.offline")}</option>
    </select>
  );
}
