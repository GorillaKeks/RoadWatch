export type FilterStatus =
  | "all"
  | "online"
  | "offline";

interface StatusFilterProps {
  value: FilterStatus;
  onChange: (value: FilterStatus) => void;
}

export function StatusFilter({
  value,
  onChange,
}: StatusFilterProps) {
  return (
    <select
      className="status-filter"
      value={value}
      onChange={(event) =>
        onChange(
          event.target.value as FilterStatus,
        )
      }
    >
      <option value="all">All</option>
      <option value="online">Online</option>
      <option value="offline">Offline</option>
    </select>
  );
}
