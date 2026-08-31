export function getDistanceInKm(
  distance: string | null | undefined,
): number | null {
  if (!distance) {
    return null;
  }

  const normalized = distance.replace(",", ".").trim().toLowerCase();

  const value = Number.parseFloat(normalized);

  if (!Number.isFinite(value)) {
    return null;
  }

  if (normalized.endsWith("km")) {
    return value;
  }

  if (normalized.endsWith("m")) {
    return value / 1000;
  }

  return null;
}
