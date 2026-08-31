export type DistanceFilterValue = "all" | "50" | "100" | "250" | "500";

interface DistanceFilterProps {
  value: DistanceFilterValue;
  onChange: (value: DistanceFilterValue) => void;
}

export function DistanceFilter({ value, onChange }: DistanceFilterProps) {
  return (
    <select
      className="distance-filter"
      value={value}
      onChange={(event) => onChange(event.target.value as DistanceFilterValue)}
      aria-label="Filter by distance"
    >
      <option value="all">All distances</option>

      <option value="50">Under 50 km</option>

      <option value="100">Under 100 km</option>

      <option value="250">Under 250 km</option>

      <option value="500">Under 500 km</option>
    </select>
  );
}
