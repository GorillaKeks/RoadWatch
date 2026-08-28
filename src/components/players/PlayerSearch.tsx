import { Search } from "lucide-react";
import { useTranslation } from "react-i18next";

interface PlayerSearchProps {
  value: string;
  onChange: (value: string) => void;
}

export function PlayerSearch({ value, onChange }: PlayerSearchProps) {
  const { t } = useTranslation();

  return (
    <div className="player-search">
      <Search size={20} />
      <input
        type="text"
        value={value}
        placeholder={t("search.placeholder")}
        onChange={(event) => onChange(event.target.value)}
      />
    </div>
  );
}
