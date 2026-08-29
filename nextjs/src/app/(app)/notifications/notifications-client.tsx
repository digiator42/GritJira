"use client";

import { useCallback, useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { apiData } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { ActivityLog, NotificationsFeed } from "@/lib/types";
import { ErrorBox } from "@/components/ui";
import { userById, formatAgo, decodeEntities } from "@/lib/format";

export function NotificationsClient() {
  const router = useRouter();
  const { currentProject, users } = useApp();
  const [feed, setFeed] = useState<NotificationsFeed | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [marking, setMarking] = useState(false);

  const load = useCallback(() => {
    if (!currentProject) return;
    apiData<NotificationsFeed>(`/api/v1/notifications?project_id=${currentProject.id}`)
      .then(setFeed)
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load notifications"));
  }, [currentProject]);

  useEffect(load, [load]);

  async function markAllRead() {
    if (!currentProject || marking) return;
    setMarking(true);
    try {
      await apiData(`/api/v1/notifications/read?project_id=${currentProject.id}`, {
        method: "POST",
      });
      load();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to mark read");
    } finally {
      setMarking(false);
    }
  }

  const items = feed?.items ?? [];

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-1 flex items-center justify-between">
        <h1 className="text-base font-semibold text-jira-text">Notifications</h1>
        <button
          type="button"
          onClick={markAllRead}
          disabled={marking || (feed?.unread ?? 0) === 0}
          className="btn-secondary text-xs"
        >
          {marking ? "…" : "Mark all read"}
        </button>
      </div>
      <p className="mb-4 text-xs text-jira-muted">
        {feed ? (
          <span>
            {feed.unread > 0 ? (
              <span className="font-medium text-jira-text">{feed.unread} unread</span>
            ) : (
              "You're all caught up."
            )}{" "}
            — things assigned to you or happening on your issues.
          </span>
        ) : (
          "Loading…"
        )}
      </p>

      {error ? <ErrorBox message={error} /> : null}

      {items.length === 0 ? (
        <p className="text-xs text-jira-faint">No notifications yet.</p>
      ) : (
        <ol className="panel divide-y divide-jira-border/60">
          {items.map((entry: ActivityLog) => (
            <li
              key={entry.id}
              className={`flex cursor-pointer gap-3 px-3 py-2.5 transition hover:bg-jira-border/30 ${
                entry.is_read ? "opacity-70" : ""
              }`}
              onClick={() => entry.issue_id && router.push(`/issues/${entry.issue_id}`)}
            >
              <span
                className={`mt-1.5 h-2 w-2 shrink-0 rounded-full ${
                  entry.is_read ? "bg-transparent" : "bg-jira-blue"
                }`}
              />
              <div className="min-w-0 flex-1">
                <p className="text-sm leading-5 text-jira-text">
                  <span className="font-medium">{userById(users, entry.actor_id)}</span>{" "}
                  <span className="text-jira-muted">{entry.action.replace(/\./g, " ")}</span>{" "}
                  {entry.issue_key ? (
                    <span className="font-medium text-jira-blue">{entry.issue_key}</span>
                  ) : null}
                </p>
                {entry.summary ? (
                  <p className="mt-0.5 truncate text-xs text-jira-muted">
                    {decodeEntities(entry.summary)}
                    {entry.detail ? ` — ${decodeEntities(entry.detail)}` : ""}
                  </p>
                ) : null}
              </div>
              <span className="shrink-0 self-center text-[10px] text-jira-faint">
                {formatAgo(entry.created_at)}
              </span>
            </li>
          ))}
        </ol>
      )}
    </div>
  );
}