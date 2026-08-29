"use client";

import { useEffect, useState } from "react";
import { apiData } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { ProjectSummary } from "@/lib/types";
import { ErrorBox } from "@/components/ui";
import { PageShimmer } from "@/components/PageShimmer";
import { IssueTypeIcon } from "@/components/IssueTypeIcon";

const STATUS_COLORS = ["#3b82f6", "#a855f7", "#f59e0b", "#ec4899", "#10b981"];

export function SummaryClient() {
  const { currentProject } = useApp();
  const [data, setData] = useState<ProjectSummary | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!currentProject) return;
    setData(null);
    setLoading(true);
    setError(null);
    apiData<ProjectSummary>(`/api/v1/projects/${currentProject.id}/summary`)
      .then(setData)
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load summary"))
      .finally(() => setLoading(false));
  }, [currentProject]);

  const maxStatus = Math.max(1, ...(data?.by_status.map((s) => s.count) ?? [0]));
  const maxType = Math.max(1, ...(data?.by_type.map((t) => t.count_total) ?? [0]));

  return (
    <div className="mx-auto max-w-4xl p-4">
      <h1 className="mb-1 text-base font-semibold text-jira-text">Summary</h1>
      <p className="mb-4 text-xs text-jira-muted">
        Status of {data?.total_issues ?? "all"} issues and the types of work in this project.
      </p>

      {error ? <ErrorBox message={error} /> : null}

      {loading && !data ? (
        <PageShimmer />
      ) : !data ? (
        <p className="text-xs text-jira-faint">Select a project to see its summary.</p>
      ) : (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
            {[
              { label: "Total issues", value: data.total_issues },
              { label: "To do / In progress", value: data.open_issues },
              { label: "Done", value: data.done_issues },
              { label: "Progress", value: `${data.total_issues > 0 ? Math.round((data.done_issues / data.total_issues) * 100) : 0}%` },
            ].map((stat) => (
              <div key={stat.label} className="panel px-3 py-2">
                <p className="text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
                  {stat.label}
                </p>
                <p className="text-lg font-semibold text-jira-text">{stat.value}</p>
              </div>
            ))}
          </div>

          <div className="panel p-3">
            <h2 className="mb-3 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Status
            </h2>
            <div className="flex h-4 w-full overflow-hidden rounded-full bg-jira-bg">
              {data.by_status
                .filter((s) => s.count > 0)
                .map((s, i) => (
                  <div
                    key={s.step_id}
                    style={{
                      width: `${(s.count / Math.max(1, data.total_issues)) * 100}%`,
                      backgroundColor: s.is_completed ? "#10b981" : STATUS_COLORS[i % STATUS_COLORS.length],
                    }}
                    title={`${s.name}: ${s.count}`}
                  />
                ))}
            </div>
            <ul className="mt-3 space-y-2">
              {data.by_status.map((s, i) => (
                <li key={s.step_id} className="flex items-center gap-2 text-sm">
                  <span
                    className="h-3 w-3 shrink-0 rounded-sm"
                    style={{ backgroundColor: s.is_completed ? "#10b981" : STATUS_COLORS[i % STATUS_COLORS.length] }}
                  />
                  <span className="min-w-24 flex-1 truncate text-jira-text">
                    {s.name}
                    {s.is_completed ? <span className="ml-1 text-[10px] uppercase text-emerald-400">done</span> : null}
                  </span>
                  <div className="hidden h-2 w-32 overflow-hidden rounded-full bg-jira-bg sm:block">
                    <div
                      className="h-full rounded-full"
                      style={{ width: `${(s.count / maxStatus) * 100}%`, backgroundColor: "#4bade9" }}
                    />
                  </div>
                  <span className="w-8 text-right text-xs font-medium text-jira-muted">{s.count}</span>
                  <span className="w-12 text-right text-[11px] text-jira-faint">
                    {s.points} pts
                  </span>
                </li>
              ))}
            </ul>
          </div>

          <div className="panel p-3">
            <h2 className="mb-1 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Types of work
            </h2>
            <p className="mb-3 text-[11px] text-jira-faint">
              {data.open_issues} open issue{data.open_issues === 1 ? "" : "s"} ({data.open_points} pts), {data.done_issues} done.
            </p>
            <ul className="space-y-2.5">
              {data.by_type.map((t) => (
                <li key={t.type_name} className="flex items-center gap-3">
                  <IssueTypeIcon iconKey={t.icon_key} color={t.color} size={16} title={t.type_name} />
                  <span className="w-32 truncate text-sm capitalize text-jira-text">{t.type_name}</span>
                  <div className="h-2 flex-1 overflow-hidden rounded-full bg-jira-bg">
                    <div
                      className="h-full rounded-full"
                      style={{ width: `${(t.count_open / maxType) * 100}%`, backgroundColor: t.color }}
                    />
                  </div>
                  <span className="w-8 text-right text-xs text-jira-muted">{t.count_open}</span>
                  <span className="w-12 text-right text-[11px] text-jira-faint">
                    {t.percent}%
                  </span>
                </li>
              ))}
            </ul>
            <p className="mt-3 text-[10px] text-jira-faint">
              Percent = open issues of this type ÷ total open issues. Includes types with no open issues.
            </p>
          </div>
        </div>
      )}
    </div>
  );
}