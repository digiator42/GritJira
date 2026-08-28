"use client";

import { useEffect, useMemo, useState } from "react";
import { useRouter } from "next/navigation";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { BoardData, Issue, Sprint } from "@/lib/types";
import { ErrorBox, Modal, Spinner } from "@/components/ui";
import { IssueCard } from "@/components/IssueCard";
import { IssueCreateForm } from "@/components/IssueCreateForm";
import { normalizeSprintStatus } from "@/lib/format";

export default function BoardClient({
  initialProjectId,
  initialSprintId,
}: {
  initialProjectId?: number;
  initialSprintId?: number;
}) {
  const { currentProject, users } = useApp();
  const router = useRouter();

  const projectId = initialProjectId || currentProject?.id;

  const [sprints, setSprints] = useState<Sprint[]>([]);
  const [sprintId, setSprintId] = useState<number | undefined>(initialSprintId);
  const [board, setBoard] = useState<BoardData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [dragIssue, setDragIssue] = useState<Issue | null>(null);
  const [moving, setMoving] = useState(false);

  useEffect(() => {
    if (!projectId) return;
    api<{ data: Sprint[] }>(`/api/v1/sprints/projects/${projectId}`)
      .then((r) => {
        const sprints = r.data;
        setSprints(sprints);
        // prefer "active", else first sprint
        const current = sprints.find((x) => normalizeSprintStatus(x.status) === "Active");
        const pick = current ?? sprints[0];
        if (pick) setSprintId((prev) => prev ?? pick.id);
      })
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load sprints"));
  }, [projectId]);

  useEffect(() => {
    if (!projectId || !sprintId) return;
    let cancelled = false;
    setLoading(true);
    api<{ data: BoardData }>(`/api/v1/board/sprints/${sprintId}?project_id=${projectId}`)
      .then((r) => {
        if (!cancelled) setBoard(r.data);
      })
      .catch((e) => {
        if (!cancelled) setError(e instanceof Error ? e.message : "Failed to load board");
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [projectId, sprintId]);

  const hasContent = useMemo(
    () => (board?.columns ?? []).some((c) => c.issues.length > 0),
    [board],
  );

  if (!projectId) {
    return (
      <div className="p-6">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }

  async function moveIssue(issueId: number, targetStepId: number) {
    const issue = (board?.columns ?? [])
      .flatMap((c) => c.issues)
      .find((i) => i.id === issueId);
    if (!issue || issue.step_id === targetStepId) return;
    setMoving(true);
    setError(null);
    try {
      // optimistic update
      if (board) {
        const next: BoardData = {
          ...board,
          columns: board.columns.map((col) => {
            const has = col.issues.some((i) => i.id === issueId);
            if (has && col.step.id !== targetStepId) {
              return { ...col, issues: col.issues.filter((i) => i.id !== issueId) };
            }
            if (!has && col.step.id === targetStepId) {
              return { ...col, issues: [...col.issues, issue] };
            }
            return col;
          }),
        };
        setBoard(next);
      }
      await api<Issue>(`/api/v1/board/issues/${issueId}/move`, {
        method: "POST",
        json: { target_step_id: targetStepId },
      });
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Move failed");
    } finally {
      setMoving(false);
      setDragIssue(null);
    }
  }

  return (
    <div className="flex h-full flex-col">
      <div className="flex shrink-0 items-center justify-between gap-3 border-b border-jira-border px-4 py-3">
        <div>
          <h1 className="text-base font-semibold text-jira-text">
            {currentProject ? `${currentProject.name} Board` : "Board"}
          </h1>
          <p className="text-xs text-jira-muted">Sprint board · drag cards between columns</p>
        </div>
        <div className="flex items-center gap-3">
          {sprints.length > 1 && (
            <select
              className="input !w-auto"
              value={sprintId ?? ""}
              onChange={(e) => {
                const id = Number(e.target.value);
                setSprintId(id);
                router.replace(`/board?project_id=${projectId}&sprint_id=${id}`);
              }}
            >
              {sprints.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name} · {normalizeSprintStatus(s.status)}
                </option>
              ))}
            </select>
          )}
          <button className="btn-primary" onClick={() => setCreateOpen(true)}>
            + Create issue
          </button>
        </div>
      </div>

      <div className="min-h-0 flex-1">
        {error ? (
          <div className="p-4">
            <ErrorBox message={error} />
          </div>
        ) : loading && !board ? (
          <Spinner label="Loading board…" />
        ) : !board || board.columns.length === 0 ? (
          <div className="p-6">
            <ErrorBox message="This project has no workflow columns yet. Open Workflow settings to add some." />
          </div>
        ) : !hasContent ? (
          <p className="p-6 text-sm text-jira-muted">
            This sprint is empty. Create an issue to get started.
          </p>
        ) : (
          <div className="flex h-full gap-3 overflow-x-auto p-4">
            {board.columns.map((col) => {
              const done = col.step.is_completed;
              return (
                <div
                  key={col.step.id}
                  className="flex w-72 shrink-0 flex-col rounded-md border border-jira-border bg-jira-panel"
                  onDragOver={(e) => e.preventDefault()}
                  onDrop={(e) => {
                    e.preventDefault();
                    if (dragIssue) void moveIssue(dragIssue.id, col.step.id);
                  }}
                >
                  <div
                    className={`flex items-center justify-between border-b border-jira-border px-3 py-2 ${
                      done ? "bg-emerald-950/30" : ""
                    }`}
                  >
                    <span className="text-xs font-semibold uppercase tracking-wide text-jira-muted">
                      {col.step.name}
                    </span>
                    <span className="text-[10px] text-jira-faint">{col.issues.length}</span>
                  </div>
                  <div className="flex flex-1 flex-col gap-2 p-2">
                    {col.issues.length === 0 ? (
                      <p className="py-4 text-center text-xs text-jira-faint">Drop issues here</p>
                    ) : (
                      col.issues.map((issue) => (
                        <IssueCard
                          key={issue.id}
                          issue={issue}
                          users={users}
                          draggable={!moving}
                          onDragStart={(e) => {
                            setDragIssue(issue);
                            e.dataTransfer.effectAllowed = "move";
                          }}
                          onClick={() => router.push(`/issues/${issue.id}`)}
                        />
                      ))
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      <Modal
        open={createOpen}
        onClose={() => setCreateOpen(false)}
        title={`Create issue in ${currentProject?.key ?? ""}`}
        wide
      >
        <IssueCreateForm
          projectId={projectId}
          sprintId={sprintId}
          onCreated={() => {
            setCreateOpen(false);
            // reload board to show the new issue
            setBoard(null);
            setLoading(true);
            api<{ data: BoardData }>(`/api/v1/board/sprints/${sprintId}?project_id=${projectId}`)
              .then((r) => setBoard(r.data))
              .catch((e) => setError(e instanceof Error ? e.message : "Failed to reload board"))
              .finally(() => setLoading(false));
          }}
        />
      </Modal>
    </div>
  );
}