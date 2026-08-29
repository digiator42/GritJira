"use client";

import { useEffect, useMemo, useState } from "react";
import { apiData } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { BurndownData, Sprint } from "@/lib/types";
import { ErrorBox, Spinner } from "@/components/ui";
import { normalizeSprintStatus } from "@/lib/format";

const W = 640;
const H = 240;
const PAD = { top: 16, right: 16, bottom: 28, left: 36 };

function linePath(points: { x: number; y: number }[]): string {
  return points
    .map((p, i) => `${i === 0 ? "M" : "L"}${p.x.toFixed(1)},${p.y.toFixed(1)}`)
    .join(" ");
}

export function BurndownClient() {
  const { currentProject } = useApp();
  const [sprints, setSprints] = useState<Sprint[] | null>(null);
  const [sprintId, setSprintId] = useState<number | null>(null);
  const [data, setData] = useState<BurndownData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!currentProject) return;
    setSprints(null);
    setData(null);
    apiData<Sprint[]>(`/api/v1/sprints/projects/${currentProject.id}`)
      .then((s) => {
        setSprints(s);
        const active = s.find((x) => x.status.toLowerCase() === "active");
        setSprintId(active?.id ?? s[s.length - 1]?.id ?? null);
      })
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load sprints"));
  }, [currentProject]);

  useEffect(() => {
    if (!currentProject || sprintId == null) return;
    setLoading(true);
    setError(null);
    apiData<BurndownData>(
      `/api/v1/board/sprints/${sprintId}/burndown?project_id=${currentProject.id}`,
    )
      .then(setData)
      .catch((e) =>
        setError(e instanceof Error ? e.message : "Failed to load burndown data"),
      )
      .finally(() => setLoading(false));
  }, [currentProject, sprintId]);

  const chart = useMemo(() => {
    if (!data) return null;
    const maxRemaining = Math.max(1, ...data.ideal.map((p) => p.remaining));
    const x = (i: number, n: number) =>
      PAD.left + (n <= 1 ? 0 : (i / (n - 1)) * (W - PAD.left - PAD.right));
    const y = (v: number) => PAD.top + (1 - v / maxRemaining) * (H - PAD.top - PAD.bottom);

    const idealPts = data.ideal.map((p, i) => ({ x: x(i, data.ideal.length), y: y(p.remaining) }));
    const actualPts = data.actual.map((p, i) => ({
      x: x(i, data.actual.length),
      y: y(p.remaining),
    }));
    const gridLines = [0, 0.25, 0.5, 0.75, 1].map((f) => {
      const gy = PAD.top + f * (H - PAD.top - PAD.bottom);
      return { gy, label: Math.round(maxRemaining * (1 - f)) };
    });
    const labels = data.ideal.map((p) => p.date.slice(5));

    return { idealPts, actualPts, gridLines, labels, maxRemaining };
  }, [data]);

  const latestActual = data?.actual[data.actual.length - 1]?.remaining ?? 0;

  return (
    <div className="mx-auto max-w-4xl p-4">
      <h1 className="mb-1 text-base font-semibold text-jira-text">Burndown</h1>
      <p className="mb-4 text-xs text-jira-muted">
        Story points remaining over the sprint versus the ideal decay line. Pull from Board data on
        demand via the backend.
      </p>

      {sprints && sprints.length > 0 ? (
        <div className="mb-4 flex items-center gap-2">
          <label className="text-xs font-semibold uppercase tracking-widest text-jira-faint">
            Sprint
          </label>
          <select
            className="input w-auto"
            value={sprintId ?? ""}
            onChange={(e) => setSprintId(Number(e.target.value))}
          >
            {sprints.map((s) => (
              <option key={s.id} value={s.id}>
                {s.name} ({normalizeSprintStatus(s.status)})
              </option>
            ))}
          </select>
        </div>
      ) : null}

      {error ? <ErrorBox message={error} /> : null}

      {loading && !data ? (
        <Spinner label="Loading burndown…" />
      ) : !data ? (
        <p className="text-xs text-jira-faint">Select a sprint to see its burndown.</p>
      ) : (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
            {[
              { label: "Committed", value: data.total_points },
              { label: "Done", value: data.done_points },
              { label: "Remaining", value: data.remaining_points },
              { label: "Complete", value: `${data.percent_done}%` },
            ].map((stat) => (
              <div key={stat.label} className="panel px-3 py-2">
                <p className="text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
                  {stat.label}
                </p>
                <p className="text-lg font-semibold text-jira-text">{stat.value}</p>
              </div>
            ))}
          </div>

          <div className="panel overflow-x-auto p-2">
            {chart ? (
              <svg
                viewBox={`0 0 ${W} ${H}`}
                className="w-full min-w-[520px]"
                role="img"
                aria-label={`Burndown chart: ${latestActual} points remaining of ${data.total_points}`}
              >
                {chart.gridLines.map(({ gy, label }, i) => (
                  <g key={i}>
                    <line
                      x1={PAD.left}
                      y1={gy}
                      x2={W - PAD.right}
                      y2={gy}
                      stroke="#1f2937"
                      strokeDasharray="3 3"
                    />
                    <text x={PAD.left - 6} y={gy + 3} textAnchor="end" fontSize="9" fill="#6b7280">
                      {label}
                    </text>
                  </g>
                ))}
                {chart.labels.map((label, i) => (
                  <text
                    key={i}
                    x={PAD.left + (i / Math.max(1, chart.labels.length - 1)) * (W - PAD.left - PAD.right)}
                    y={H - 8}
                    textAnchor="middle"
                    fontSize="9"
                    fill="#6b7280"
                  >
                    {label}
                  </text>
                ))}
                <path d={linePath(chart.idealPts)} fill="none" stroke="#6b7280" strokeWidth="2" strokeDasharray="5 3" />
                <path d={linePath(chart.actualPts)} fill="none" stroke="#3b82f6" strokeWidth="2.5" />
                {chart.actualPts.map((p, i) => (
                  <circle key={i} cx={p.x} cy={p.y} r="3.5" fill="#3b82f6" />
                ))}
              </svg>
            ) : null}
            <div className="mt-1 flex gap-4 px-2 text-[10px] text-jira-muted">
              <span className="inline-flex items-center gap-1.5">
                <span className="inline-block h-0.5 w-4 bg-jira-blue" /> Actual remaining
              </span>
              <span className="inline-flex items-center gap-1.5">
                <span className="inline-block h-0.5 w-4 border-t-2 border-dashed border-jira-faint" /> Ideal
              </span>
            </div>
          </div>

          <div className="panel p-3">
            <h2 className="mb-2 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Points per column
            </h2>
            <div className="flex h-4 w-full overflow-hidden rounded-full bg-jira-bg">
              {data.columns
                .filter((c) => c.points > 0)
                .map((c, i) => (
                  <div
                    key={c.id}
                    style={{
                      width: `${(c.points / Math.max(1, data.total_points)) * 100}%`,
                      backgroundColor: c.is_completed ? "#10b981" : ["#3b82f6", "#a855f7", "#f59e0b"][i % 3],
                    }}
                    title={`${c.name}: ${c.points} pts`}
                  />
                ))}
            </div>
            <div className="mt-2 flex flex-wrap gap-3 text-[10px] text-jira-muted">
              {data.columns.map((c, i) => (
                <span key={c.id} className="inline-flex items-center gap-1.5">
                  <span
                    className="inline-block h-2.5 w-2.5 rounded-sm"
                    style={{
                      backgroundColor: c.is_completed ? "#10b981" : ["#3b82f6", "#a855f7", "#f59e0b"][i % 3],
                      opacity: c.points === 0 ? 0.25 : 1,
                    }}
                  />
                  {c.name}: {c.points} (×{c.count})
                </span>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}