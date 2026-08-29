"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { apiData } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { ActivityLog } from "@/lib/types";
import { ErrorBox } from "@/components/ui";
import { userById, formatAgo, decodeEntities } from "@/lib/format";

const ACTION_META: Record<string, { label: string; color: string; glyph: string }> = {
  created: { label: "created", color: "text-green-400", glyph: "+" },
  updated: { label: "updated", color: "text-blue-400", glyph: "✎" },
  moved: { label: "moved", color: "text-purple-400", glyph: "⇄" },
  assigned: { label: "assigned", color: "text-amber-400", glyph: "@" },
  commented: { label: "commented on", color: "text-cyan-400", glyph: "💬" },
  deleted: { label: "deleted", color: "text-red-400", glyph: "✖" },
  "sprint.started": { label: "started sprint", color: "text-green-400", glyph: "▶" },
  "sprint.completed": { label: "completed sprint", color: "text-jira-faint", glyph: "✓" },
};

function metaFor(action: string) {
  return (
    ACTION_META[action] ?? {
      label: action.replace(/\./g, " "),
      color: "text-jira-muted",
      glyph: "•",
    }
  );
}

export function ActivityClient() {
  const router = useRouter();
  const { currentProject, users } = useApp();
  const [items, setItems] = useState<ActivityLog[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!currentProject) return;
    setItems(null);
    apiData<ActivityLog[]>(`/api/v1/activity/projects/${currentProject.id}`)
      .then(setItems)
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load activity"));
  }, [currentProject]);

  return (
    <div className="mx-auto max-w-3xl p-4">
      <h1 className="mb-1 text-base font-semibold text-jira-text">Activity</h1>
      <p className="mb-4 text-xs text-jira-muted">
        Recent changes across {currentProject?.name ?? "the project"} — an audit trail of issues,
        comments and sprints.
      </p>

      {error ? <ErrorBox message={error} /> : null}

      {!items ? (
        <p className="text-xs text-jira-faint">Loading activity…</p>
      ) : items.length === 0 ? (
        <p className="text-xs text-jira-faint">
          No activity yet. Your changes will appear here as you work.
        </p>
      ) : (
        <ol className="panel divide-y divide-jira-border/60">
          {items.map((entry) => {
            const meta = metaFor(entry.action);
            return (
              <li key={entry.id} className="flex gap-3 px-3 py-2.5">
                <span
                  className={`flex h-6 w-6 shrink-0 items-center justify-center rounded-full border border-jira-border bg-jira-bg text-xs ${meta.color}`}
                  title={meta.label}
                >
                  {meta.glyph}
                </span>
                <div className="min-w-0 flex-1">
                  <p className="text-sm leading-5 text-jira-text">
                    <span className="font-medium">{userById(users, entry.actor_id)}</span>{" "}
                    <span className="text-jira-muted">{meta.label}</span>{" "}
                    {entry.issue_key ? (
                      <button
                        type="button"
                        onClick={() => entry.issue_id && router.push(`/issues/${entry.issue_id}`)}
                        className={`font-medium underline-offset-2 hover:underline ${meta.color}`}
                      >
                        {entry.issue_key}
                      </button>
                    ) : (
                      <span className="font-medium text-jira-text">
                        {decodeEntities(entry.summary) || "sprint"}
                      </span>
                    )}
                    {entry.summary ? (
                      <span className="text-jira-muted"> — {decodeEntities(entry.summary)}</span>
                    ) : null}
                  </p>
                  {entry.detail ? (
                    <p className="mt-0.5 text-xs text-jira-faint">{decodeEntities(entry.detail)}</p>
                  ) : null}
                </div>
                <span className="shrink-0 self-center text-[10px] text-jira-faint">
                  {formatAgo(entry.created_at)}
                </span>
              </li>
            );
          })}
        </ol>
      )}
    </div>
  );
}