import type { ReactNode } from "react";
import { DEFAULT_ISSUE_TYPE_STYLES, FALLBACK_ISSUE_TYPE_STYLE, type IssueType } from "@/lib/types";

// Resolve display style (icon key + color + label) for a stored issue_type
// string, preferring the project's configured issue types catalog.
export function resolveIssueType(
  issueType: string,
  issueTypes?: IssueType[],
): { icon_key: string; color: string; label: string } {
  const key = (issueType || "").toLowerCase();
  const cat = issueTypes?.find((t) => t.name.toLowerCase() === key);
  if (cat) {
    return { icon_key: cat.icon_key, color: cat.color, label: cat.name };
  }
  const fallback = DEFAULT_ISSUE_TYPE_STYLES[key] ?? FALLBACK_ISSUE_TYPE_STYLE;
  return { icon_key: fallback.icon_key, color: fallback.color, label: key || "task" };
}

const GLYPHS: Record<string, ReactNode> = {
  bug: (
    <>
      <circle cx="12" cy="14" r="5.5" />
      <line x1="12" y1="8.5" x2="12" y2="6.5" />
      <circle cx="12" cy="5.5" r="1.1" fill="currentColor" stroke="none" />
      <line x1="7" y1="10" x2="4" y2="8" />
      <line x1="17" y1="10" x2="20" y2="8" />
      <line x1="6.5" y1="16" x2="3.5" y2="18" />
      <line x1="17.5" y1="16" x2="20.5" y2="18" />
    </>
  ),
  story: (
    <>
      <path d="M4 6.5A2.5 2.5 0 016.5 4H9v14H6.5A2.5 2.5 0 014 20.5z" />
      <path d="M20 6.5A2.5 2.5 0 0017.5 4H15v14h2.5a2.5 2.5 0 012.5 2.5z" />
      <line x1="12" y1="4" x2="12" y2="18" />
    </>
  ),
  task: (
    <>
      <circle cx="12" cy="12" r="8.5" />
      <path d="M8.5 12.5l2.5 2.5 4.5-5" strokeLinecap="round" strokeLinejoin="round" />
    </>
  ),
  epic: (
    <>
      <line x1="6" y1="3" x2="6" y2="21" />
      <path d="M6 5c3-2 6 1.5 9 0v7c-3 1.5-6-2-9 0z" strokeLinejoin="round" />
    </>
  ),
  subtask: (
    <>
      <rect x="9" y="4" width="11" height="11" rx="1.8" />
      <rect x="4" y="9" width="11" height="11" rx="1.8" />
    </>
  ),
  test: (
    <>
      <path d="M10 3h4" />
      <path d="M11 3v5.2L5.6 16.6A2 2 0 007.3 20h9.4a2 2 0 001.7-3.4L13 8.2V3" strokeLinejoin="round" />
      <line x1="8" y1="15" x2="16" y2="15" />
    </>
  ),
  "test-execution": (
    <>
      <rect x="3.5" y="3.5" width="17" height="17" rx="3" />
      <path d="M10.5 9.2l5 2.8-5 2.8z" fill="currentColor" stroke="none" />
    </>
  ),
  "test-plan": (
    <>
      <rect x="6" y="4" width="12" height="17" rx="2" />
      <path d="M9 3.5h6v2.5H9z" />
      <path d="M9.5 11l2 2 3.5-4" strokeLinecap="round" strokeLinejoin="round" />
    </>
  ),
  "test-set": (
    <>
      <rect x="3" y="4" width="14" height="12" rx="1.6" />
      <rect x="6.5" y="8" width="14" height="12" rx="1.6" />
    </>
  ),
  precondition: (
    <>
      <path d="M12 3l8 9-8 9-8-9z" strokeLinejoin="round" />
      <path d="M9.5 12l1.8 1.8 3.2-3.6" strokeLinecap="round" strokeLinejoin="round" />
    </>
  ),
};

export function IssueTypeIcon({
  iconKey,
  color,
  size = 16,
  title,
}: {
  iconKey: string;
  color: string;
  size?: number;
  title?: string;
}) {
  const glyph = GLYPHS[iconKey] ?? GLYPHS.task;
  return (
    <span
      title={title}
      className="inline-flex shrink-0 items-center justify-center rounded"
      style={{ width: size, height: size, backgroundColor: color }}
    >
      <svg
        width={Math.round(size * 0.62)}
        height={Math.round(size * 0.62)}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        className="text-white"
      >
        {glyph}
      </svg>
    </span>
  );
}

export function IssueTypeBadge({
  type,
  issueTypes,
  size = 16,
  className = "",
}: {
  type: string;
  issueTypes?: IssueType[];
  size?: number;
  className?: string;
}) {
  const style = resolveIssueType(type, issueTypes);
  return (
    <span
      className={`inline-flex items-center gap-1.5 text-xs text-jira-muted ${className}`}
      title={style.label}
    >
      <IssueTypeIcon iconKey={style.icon_key} color={style.color} size={size} />
      <span className="capitalize">{style.label}</span>
    </span>
  );
}

export const ISSUE_TYPE_ICON_KEYS = [
  "bug",
  "story",
  "task",
  "epic",
  "subtask",
  "test",
  "test-execution",
  "test-set",
  "test-plan",
  "precondition",
] as const;