import { useTranslation } from "react-i18next";
import { Languages } from "lucide-react";

export function LanguageSelector() {
  const { i18n } = useTranslation();

  return (
    <div className="language-selector">
      <Languages size={18} />
      <select
        value={i18n.language.startsWith("de") ? "de" : "en"}
        onChange={(event) => i18n.changeLanguage(event.target.value)}
        aria-label="Language"
      >
        <option value="de">DE</option>
        <option value="en">EN</option>
      </select>
    </div>
  );
}
