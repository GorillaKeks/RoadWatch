import { Search } from "lucide-react";

interface PlayerSearchProps {
  value: string;
  onChange: (value: string) => void;
}

export function PlayerSearch({
  value,
  onChange,
}: PlayerSearchProps) {
  return (
    <div className="player-search">
      <Search size={20} />

      <input
        type="text"
        value={value}
        placeholder="Search players..."
        onChange={(event) =>
          onChange(event.target.value)
        }
      />
    </div>
  );
}
