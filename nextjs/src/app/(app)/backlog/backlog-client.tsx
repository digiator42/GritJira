"use client";

import { useCallback, useEffect, useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { BacklogData, Issue, Sprint } from "@/lib/types";
import { Avatar, EmptyState, ErrorBox, Field, Modal, SprintStatusBadge } from "@/components/ui";
import { IssueCard } from "@/components/IssueCard";
import { userById, normalizeSprintStatus } from "@/lib/format";
import { useRouter } from "next/navigation";

export default function BacklogClient({
  initialProjectId,
}: {
  initialProjectId?: number;
}) {
  const { currentProject, users } = useApp();
  const router = useRouter();
  const projectId = initialProjectId || currentProject?.id;

  const [data, setData] = useState<BacklogData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);

  const load = useCallback(() => {
    if (!projectId) return;
    setLoading(true);
    api<{ data: BacklogData }>(`/api/v1/backlog/projects/${projectId}`)
      .then((r) => setData(r.data))
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load backlog"))
      .finally(() => setLoading(false));
  }, [projectId]);

  useEffect(() => {
    load();
  }, [load]);

  async function assign(issueId: number, sprintId: number | null) {
    try {
      await api(`/api/v1/backlog/issues/${issueId}/assign-sprint`, {
        method: "POST",
        json: { sprint_id: sprintId },
      });
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Assignment failed");
    }
  }

  async function sprintAction(action: "start" | "complete" | "delete", id: number) {
    try {
      if (action === "delete") {
        await api(`/api/v1/sprints/${id}`, { method: "DELETE" });
      } else {
        await api(`/api/v1/sprints/${id}/${action}`, { method: "POST" });
      }
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Sprint action failed");
    }
  }

  if (!projectId) {
    return (
      <div className="p-6">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Backlog</h1>
          <p className="text-xs text-jira-muted">Unassigned issues and sprints</p>
        </div>
        <button className="btn-primary" onClick={() => setCreateOpen(true)}>
          + Create sprint
        </button>
      </div>

      {error ? <ErrorBox message={error} /> : null}

      {loading && !data ? (
        <div className="text-center text-sm text-jira-muted">Loading…</div>
      ) : data ? (
        <div className="grid gap-4 lg:grid-cols-2">
          {/* Sprints */}
          <section className="panel p-3">
            <h2 className="mb-3 px-1 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Sprints ({data.sprints.length})
            </h2>
            {data.sprints.length === 0 ? (
              <EmptyState title="No sprints yet" hint="Create a sprint to plan your work." />
            ) : (
              <div className="space-y-2">
                {data.sprints.map((s: Sprint) => (
                  <div key={s.id} className="rounded-md border border-jira-border bg-jira-bg p-3">
                    <div className="mb-1 flex items-center gap-2">
                      <span className="text-sm font-semibold text-jira-text">{s.name}</span>
                      <SprintStatusBadge status={s.status} />
                    </div>
                    <p className="mb-2 text-xs text-jira-muted">
                      {s.goal || "No goal set."}
                    </p>
                    <div className="flex flex-wrap gap-1.5">
                      {normalizeSprintStatus(s.status) === "Planning" ? (
                        <button
                          className="btn-secondary !px-2 !py-1 !text-xs"
                          onClick={() => void sprintAction("start", s.id)}
                        >
                          Start
                        </button>
                      ) : normalizeSprintStatus(s.status) === "Active" ? (
                        <button
                          className="btn-secondary !px-2 !py-1 !text-xs"
                          onClick={() => void sprintAction("complete", s.id)}
                        >
                          Complete
                        </button>
                      ) : null}
                      {normalizeSprintStatus(s.status) !== "Active" ? (
                        <button
                          className="btn-danger !px-2 !py-1 !text-xs"
                          onClick={() => {
                            if (confirm(`Delete sprint "${s.name}"?`)) void sprintAction("delete", s.id);
                          }}
                        >
                          Delete
                        </button>
                      ) : (
                        <button
                          className="btn-secondary !px-2 !py-1 !text-xs"
                          onClick={() =>
                            router.push(`/board?project_id=${projectId}&sprint_id=${s.id}`)
                          }
                        >
                          Open board
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </section>

          {/* Backlog issues */}
          <section className="panel p-3">
            <h2 className="mb-3 px-1 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Backlog ({data.backlog_issues.length})
            </h2>
            {data.backlog_issues.length === 0 ? (
              <EmptyState title="Backlog is empty" hint="Drag new issues across by creating them." />
            ) : (
              <div className="space-y-2">
                {data.backlog_issues.map((issue: Issue) => (
                  <div
                    key={issue.id}
                    className="rounded-md border border-jira-border bg-jira-bg p-2"
                  >
                    <IssueCard issue={issue} users={users} onClick={() => router.push(`/issues/${issue.id}`)} />
                    <div className="mt-2 flex items-center justify-between px-0.5">
                      <span className="text-[10px] text-jira-faint">
                        {userById(users, issue.assignee_id)}
                      </span>
                      <select
                        className="input !w-48 !py-1 !text-xs"
                        value=""
                        onChange={(e) => {
                          const val = e.target.value;
                          if (val) {
                            void assign(issue.id, Number(val));
                            e.target.value = "";
                          }
                        }}
                      >
                        <option value="">Assign to sprint…</option>
                        {data.sprints.map((s) => (
                          <option key={s.id} value={s.id}>
                            {s.name}
                          </option>
                        ))}
                        <option value="null">Remove from sprint (backlog)</option>
                      </select>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </section>
        </div>
      ) : null}

      <Modal open={createOpen} onClose={() => setCreateOpen(false)} title="Create sprint">
        <SprintCreateForm
          projectId={projectId}
          onCreated={() => {
            setCreateOpen(false);
            load();
          }}
        />
      </Modal>
    </div>
  );
}

function SprintCreateForm({
  projectId,
  onCreated,
}: {
  projectId: number;
  onCreated: () => void;
}) {
  const [name, setName] = useState("");
  const [goal, setGoal] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setBusy(true);
    try {
      await api<Sprint>(`/api/v1/sprints/projects/${projectId}`, {
        method: "POST",
        json: { name, goal: goal || undefined },
      });
      onCreated();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to create sprint");
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit} className="space-y-3">
      <Field label="Sprint name">
        <input required className="input" value={name} onChange={(e) => setName(e.target.value)} placeholder="Sprint 1" />
      </Field>
      <Field label="Goal">
        <input className="input" value={goal} onChange={(e) => setGoal(e.target.value)} placeholder="What is the sprint goal?" />
      </Field>
      {error ? <p className="text-xs text-red-300">{error}</p> : null}
      <div className="flex justify-end gap-2">
        <button type="button" className="btn-secondary" onClick={onCreated}>
          Cancel
        </button>
        <button type="submit" disabled={busy} className="btn-primary">
          {busy ? "Creating…" : "Create sprint"}
        </button>
      </div>
    </form>
  );
}