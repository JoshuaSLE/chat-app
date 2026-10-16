export function formatTime(dateStr: string): string {
  const date = new Date(dateStr);
  if (isNaN(date.getTime())) return "";

  const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const isUtc = !timeZone;

  const effectiveTimeZone = isUtc ? "UTC" : timeZone;

  const formatted = date.toLocaleString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
    timeZone: effectiveTimeZone,
  });

  const label = isUtc
    ? "UTC"
    : (timeZone.split("/").pop()?.replace(/_/g, " ") ?? "UTC");

  return `${formatted} (${label})`;
}
