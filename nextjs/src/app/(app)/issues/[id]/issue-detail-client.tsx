"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Comment, Issue, IssueDetail, WorkflowStep } from "@/lib/types";
import { Avatar, ErrorBox, Field, PriorityBadge, Spinner, TypeBadge } from "@/components/ui";
import { formatDate, userById } from "@/lib/format";

export function IssueDetailClient({ id }: { id: number }) {
  const router = useRouter();
  const { users } = useApp();

  const [detail, setDetail] = useState<IssueDetail | null>(null);
  const [steps, setSteps] = useState<WorkflowStep[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const issue = detail?.issue;

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    api<{ data: IssueDetail }>(`/api/v1/issues/${id}`)
      .then((r) => {
        if (cancelled) return;
        const d = r.data;
        setDetail(d);
        return api<{ data: WorkflowStep[] }>(`/api/v1/projects/${d.issue.project_id}/workflow`)
          .then((rw) => setSteps(rw.data))
          .catch(() => setSteps([]));
      })
      .catch((e) => {
        if (!cancelled) setError(e instanceof Error ? e.message : "Failed to load issue");
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [id]);

  // Editable local state (synced when issue loads/reloads)
  const [drafts, setDrafts] = useState<{
    summary: string;
    description: string;
    priority: number;
    issue_type: string;
    story_points: string | null;
    assignee_id: string;
  } | null>(null);

  useEffect(() => {
    if (issue && !drafts) {
      setDrafts({
        summary: issue.summary,
        description: issue.description ?? "",
        priority: issue.priority,
        issue_type: issue.issue_type,
        story_points: issue.story_points != null ? String(issue.story_points) : null,
        assignee_id: issue.assignee_id != null ? String(issue.assignee_id) : "",
      });
    }
  }, [issue, drafts]);

  if (loading && !detail) {
    return (
      <div className="p-4">
        <Spinner label="Loading issue…" />
      </div>
    );
  }

  if (error && !detail) {
    return (
      <div className="p-4">
        <ErrorBox message={error} />
      </div>
    );
  }

  if (!detail || !issue || !drafts) {
    return null;
  }

  async function patch(payload: Record<string, unknown>) {
    try {
      const r = await api<{ data: Issue }>(`/api/v1/issues/${id}`, {
        method: "PATCH",
        json: payload,
      });
      const updated = r.data;
      setDetail((d) => (d ? { ...d, issue: updated } : d));
      setDrafts((d0) =>
        d0 ? { ...d0, ...normalizeDrafts(updated) } : d0,
      );
      setError(null);
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Update failed");
    }
  }

  function normalizeDrafts(u: Issue) {
    return {
      summary: u.summary,
      description: u.description ?? "",
      priority: u.priority,
      issue_type: u.issue_type,
      story_points: u.story_points != null ? String(u.story_points) : null,
      assignee_id: u.assignee_id != null ? String(u.assignee_id) : "",
    };
  }

  async function moveToStep(stepId: number) {
    try {
      const r = await api<{ data: Issue }>(`/api/v1/issues/${id}/step`, {
        method: "PATCH",
        json: { target_step_id: stepId },
      });
      setDetail((d) => (d ? { ...d, issue: r.data } : d));
      setError(null);
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Move failed");
    }
  }

  async function setAssignee(value: string) {
    const assignee_id = value === "" ? null : Number(value);
    try {
      const r = await api<{ data: Issue }>(`/api/v1/issues/${id}/assignee`, {
        method: "PATCH",
        json: { assignee_id },
      });
      setDetail((d) => (d ? { ...d, issue: r.data } : d));
      setDrafts((d0) => (d0 ? { ...d0, assignee_id: value } : d0));
      setError(null);
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Assignee update failed");
    }
  }

  async function del() {
    if (!issue || !confirm(`Delete ${issue.key} "${issue.summary}"?`)) return;
    try {
      await api(`/api/v1/issues/${id}`, { method: "DELETE" });
      router.replace("/board");
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Delete failed");
    }
  }

  return (
    <div className="mx-auto max-w-4xl p-4">
      {error ? (
        <div className="mb-4">
          <ErrorBox message={error} />
        </div>
      ) : null}

      <div className="mb-3 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Link href="/board" className="text-sm text-jira-muted hover:text-jira-text">
            ← Back
          </Link>
          <span className="text-sm font-semibold text-jira-faint">{issue.key}</span>
          <TypeBadge type={issue.issue_type} />
          <PriorityBadge value={issue.priority} />
        </div>
        <button className="btn-danger" onClick={() => void del()}>
          Delete
        </button>
      </div>

      <div className="grid gap-4 lg:grid-cols-[1fr_240px]">
        <div className="space-y-4">
          <div className="panel p-4">
            <label className="label">Summary</label>
            <input
              className="input !text-base"
              value={drafts.summary}
              onChange={(e) => setDrafts((d) => (d ? { ...d, summary: e.target.value } : d))}
              onBlur={() => {
                if (drafts.summary !== issue.summary) void patch({ summary: drafts.summary });
              }}
            />

            <div className="mt-3">
              <label className="label">Description</label>
              <textarea
                className="input min-h-[140px] resize-y"
                value={drafts.description}
                onChange={(e) => setDrafts((d) => (d ? { ...d, description: e.target.value } : d))}
                onBlur={() => {
                  if (drafts.description !== (issue.description ?? ""))
                    void patch({ description: drafts.description || null });
                }}
                placeholder="Add a description…"
              />
            </div>
          </div>

          <div className="panel p-4">
            <h3 className="mb-3 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Workflow
            </h3>
            {steps.length === 0 ? (
              <p className="text-xs text-jira-faint">No workflow steps for this project.</p>
            ) : (
              <div className="flex flex-wrap gap-1.5">
                {steps.map((s) => {
                  const active = s.id === issue.step_id;
                  return (
                    <button
                      key={s.id}
                      onClick={() => void moveToStep(s.id)}
                      className={`rounded-md border px-3 py-1.5 text-xs font-medium transition ${
                        active
                          ? "border-jira-blue bg-jira-blue/20 text-white"
                          : "border-jira-border bg-jira-bg text-jira-muted hover:border-jira-blue/50"
                      }`}
                    >
                      {s.name}
                    </button>
                  );
                })}
              </div>
            )}
          </div>

          <div className="panel p-4">
            <h3 className="mb-3 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Comments ({detail.comments.length})
            </h3>
            <div className="space-y-3">
              {detail.comments.length === 0 ? (
                <p className="text-xs text-jira-faint">No comments yet.</p>
              ) : (
                detail.comments.map((c: Comment) => (
                  <div key={c.id} className="rounded-md border border-jira-border bg-jira-bg p-3">
                    <div className="mb-1 flex items-center gap-2">
                      <Avatar name={userById(users, c.author_id)} size={20} />
                      <span className="text-xs font-medium text-jira-text">
                        {userById(users, c.author_id)}
                      </span>
                      <span className="text-[10px] text-jira-faint">
                        {formatDate(c.created_at)}
                      </span>
                    </div>
                    <p className="whitespace-pre-wrap text-sm text-jira-text">{c.body}</p>
                  </div>
                ))
              )}
              <CommentForm
                onSubmit={async (body) => {
                  try {
                    const rc = await api<{ data: Comment }>(`/api/v1/issues/${id}/comments`, {
                      method: "POST",
                      json: { body },
                    });
                    setDetail((d) =>
                      d ? { ...d, comments: [...d.comments, rc.data] } : d,
                    );
                  } catch (e) {
                    setError(e instanceof ApiError ? e.message : "Comment failed");
                  }
                }}
              />
            </div>
          </div>
        </div>

        <div className="space-y-3">
          <div className="panel p-3">
            <Field label="Assignee">
              <select
                className="input"
                value={drafts.assignee_id}
                onChange={(e) => void setAssignee(e.target.value)}
              >
                <option value="">Unassigned</option>
                {users.map((u) => (
                  <option key={u.id} value={u.id}>
                    {u.username}
                  </option>
                ))}
              </select>
            </Field>
            <div className="mt-3">
              <Field label="Priority">
                <select
                  className="input"
                  value={drafts.priority}
                  onChange={(e) => void patch({ priority: Number(e.target.value) })}
                >
                  {[1, 2, 3, 4, 5].map((p) => (
                    <option key={p} value={p}>
                      {["Highest", "High", "Medium", "Low", "Lowest"][p - 1]}
                    </option>
                  ))}
                </select>
              </Field>
            </div>
            <div className="mt-3">
              <Field label="Type">
                <select
                  className="input"
                  value={drafts.issue_type}
                  onChange={(e) => void patch({ issue_type: e.target.value })}
                >
                  {["story", "bug", "task", "epic", "subtask"].map((t) => (
                    <option key={t} value={t}>
                      {t}
                    </option>
                  ))}
                </select>
              </Field>
            </div>
            <div className="mt-3">
              <Field label="Story points">
                <input
                  type="number"
                  min={0}
                  className="input"
                  value={drafts.story_points ?? ""}
                  onChange={(e) =>
                    setDrafts((d) =>
                      d ? { ...d, story_points: e.target.value === "" ? null : e.target.value } : d,
                    )
                  }
                  onBlur={() => {
                    const v =
                      drafts.story_points === null || drafts.story_points === ""
                        ? null
                        : Number(drafts.story_points);
                    if (v !== issue.story_points) void patch({ story_points: v });
                  }}
                />
              </Field>
            </div>
          </div>

          <div className="panel p-3 text-xs text-jira-faint">
            <p>
              Reporter: <span className="text-jira-muted">{userById(users, issue.reporter_id)}</span>
            </p>
            <p className="mt-1">
              Created: <span className="text-jira-muted">{formatDate(issue.created_at)}</span>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

function CommentForm({ onSubmit }: { onSubmit: (body: string) => Promise<void> }) {
  const [body, setBody] = useState("");
  const [busy, setBusy] = useState(false);

  return (
    <form
      onSubmit={async (e) => {
        e.preventDefault();
        if (!body.trim() || busy) return;
        setBusy(true);
        try {
          await onSubmit(body);
          setBody("");
        } finally {
          setBusy(false);
        }
      }}
    >
      <textarea
        className="input min-h-[60px]"
        value={body}
        onChange={(e) => setBody(e.target.value)}
        placeholder="Write a comment…"
      />
      <div className="mt-2 flex justify-end">
        <button type="submit" disabled={busy || !body.trim()} className="btn-primary !py-1.5 !text-xs">
          {busy ? "Posting…" : "Add comment"}
        </button>
      </div>
    </form>
  );
}