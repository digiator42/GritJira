export function formatDate(value: string | null | undefined): string {
  if (!value) return "—";
  const d = new Date(value);
  if (Number.isNaN(d.getTime())) return value.slice(0, 10);
  return d.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function initials(name: string): string {
  return name
    .split(/[\s@._-]+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((p) => p[0]?.toUpperCase())
    .join("");
}

export function priorityLabel(value: number): string {
  switch (value) {
    case 1:
      return "Highest";
    case 2:
      return "High";
    case 3:
      return "Medium";
    case 4:
      return "Low";
    case 5:
      return "Lowest";
    default:
      return "—";
  }
}

export function normalizeSprintStatus(status: string): string {
  const lower = status.toLowerCase();
  if (["active", "in_progress"].includes(lower)) return "Active";
  if (["completed", "done", "complete"].includes(lower)) return "Completed";
  if (["planning", "future", "planned"].includes(lower)) return "Planning";
  return status;
}

export function userById(users: { id: number; username: string }[], id: number | null | undefined): string {
  if (id == null) return "Unassigned";
  return users.find((u) => u.id === id)?.username ?? `User #${id}`;
}

// The API stores text HTML-escaped (&#x27;, &amp;, &#x2F;, ...). Decode for display.
export function decodeEntities(value: string | null | undefined): string {
  if (value == null) return "";
  return value
    .replace(/&#x27;/gi, "'")
    .replace(/&apos;/gi, "'")
    .replace(/&quot;/gi, '"')
    .replace(/&#x2F;/gi, "/")
    .replace(/&#x3C;/gi, "<")
    .replace(/&#x3E;/gi, ">")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&amp;/g, "&")
    .replace(/&#(\d+);/g, (_, n) => String.fromCharCode(Number(n)));
}