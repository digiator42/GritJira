"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Attachment, Comment, Issue, IssueDetail, WorkflowStep } from "@/lib/types";
import { Avatar, ErrorBox, Field, PriorityBadge } from "@/components/ui";
import { PageShimmer } from "@/components/PageShimmer";
import { IssueTypeBadge } from "@/components/IssueTypeIcon";
import { AssigneePicker, TypePicker } from "@/components/pickers";
import { formatDate, userById, decodeEntities } from "@/lib/format";

export function IssueDetailClient({ id }: { id: number }) {
  const router = useRouter();
  const { users, issueTypes } = useApp();

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
    due_date: string | null;
  } | null>(null);

  useEffect(() => {
    if (issue && !drafts) {
      setDrafts({
        summary: decodeEntities(issue.summary),
        description: decodeEntities(issue.description ?? ""),
        priority: issue.priority,
        issue_type: issue.issue_type,
        story_points: issue.story_points != null ? String(issue.story_points) : null,
        assignee_id: issue.assignee_id != null ? String(issue.assignee_id) : "",
        due_date: issue.due_date ?? "",
      });
    }
  }, [issue, drafts]);

  const [timeDraft, setTimeDraft] = useState<{ estimate: string; log: string }>({
    estimate: "",
    log: "",
  });

  useEffect(() => {
    if (issue) {
      setTimeDraft((t) => ({
        ...t,
        estimate: issue.time_estimate_minutes != null ? String(issue.time_estimate_minutes) : "",
      }));
    }
  }, [id]);

  async function logTime() {
    const mins = Number(timeDraft.log);
    if (!mins || mins <= 0) return;
    try {
      const r = await api<{ data: Issue }>(`/api/v1/issues/${id}/time`, {
        method: "POST",
        json: { minutes: mins },
      });
      setDetail((d) => (d ? { ...d, issue: r.data } : d));
      setTimeDraft((t) => ({ ...t, log: "" }));
      setError(null);
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Log time failed");
    }
  }

  if (loading && !detail) {
    return (
      <div className="p-4">
        <PageShimmer />
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
      summary: decodeEntities(u.summary),
      description: decodeEntities(u.description ?? ""),
      priority: u.priority,
      issue_type: u.issue_type,
      story_points: u.story_points != null ? String(u.story_points) : null,
      assignee_id: u.assignee_id != null ? String(u.assignee_id) : "",
      due_date: u.due_date ?? "",
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
    if (!issue || !confirm(`Delete ${issue.key} "${decodeEntities(issue.summary)}"?`)) return;
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
          <IssueTypeBadge type={issue.issue_type} issueTypes={issueTypes} />
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
                if (drafts.summary !== decodeEntities(issue.summary)) void patch({ summary: drafts.summary });
              }}
            />

            <div className="mt-3">
              <label className="label">Description</label>
              <textarea
                className="input min-h-[140px] resize-y"
                value={drafts.description}
                onChange={(e) => setDrafts((d) => (d ? { ...d, description: e.target.value } : d))}
                onBlur={() => {
                  if (drafts.description !== decodeEntities(issue.description ?? ""))
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
                    <p className="whitespace-pre-wrap text-sm text-jira-text">{decodeEntities(c.body)}</p>
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

          <AttachmentsPanel
            issueId={id}
            attachments={detail.attachments ?? []}
            onChanged={(updater) =>
              setDetail((d) => (d ? { ...d, attachments: updater(d.attachments ?? []) } : d))
            }
            onError={(msg) => setError(msg)}
          />
        </div>

        <div className="space-y-3">
          <div className="panel p-3">
            <Field label="Assignee">
              <AssigneePicker
                users={users}
                value={drafts.assignee_id}
                onChange={(v) => void setAssignee(v)}
              />
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
                <TypePicker
                  options={issueTypes}
                  value={drafts.issue_type}
                  onChange={(v) => void patch({ issue_type: v })}
                />
              </Field>
            </div>
            <div className="mt-3">
              <Field label="Due date">
                <input
                  type="date"
                  className="input"
                  value={drafts.due_date ?? ""}
                  onChange={(e) =>
                    setDrafts((d) => (d ? { ...d, due_date: e.target.value } : d))
                  }
                  onBlur={() => {
                    const v = drafts.due_date === "" ? null : drafts.due_date;
                    if (v !== issue.due_date) void patch({ due_date: v });
                  }}
                />
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
            <div className="mt-3">
              <Field label="Original estimate">
                <div className="flex items-center gap-2">
                  <input
                    type="number"
                    min={0}
                    className="input"
                    value={timeDraft.estimate}
                    onChange={(e) =>
                      setTimeDraft((t) => ({ ...t, estimate: e.target.value }))
                    }
                    onBlur={() => {
                      const v =
                        timeDraft.estimate === "" ? null : Number(timeDraft.estimate);
                      if (v !== issue.time_estimate_minutes) void patch({ time_estimate_minutes: v });
                    }}
                    placeholder="minutes"
                  />
                  <span className="shrink-0 text-xs text-jira-faint">{fmtMins(issue.time_estimate_minutes)}</span>
                </div>
              </Field>
            </div>
            <div className="mt-3">
              <Field label="Time spent">
                <p className="text-sm text-jira-text">{fmtMins(issue.time_spent_minutes)} logged</p>
                <p className="mt-0.5 text-[11px] text-jira-faint">
                  Remaining:{" "}
                  {fmtMins(
                    (issue.time_estimate_minutes ?? 0) - issue.time_spent_minutes,
                  )}
                </p>
              </Field>
            </div>
            <div className="mt-3">
              <Field label="Log time">
                <div className="flex items-center gap-2">
                  <input
                    type="number"
                    min={1}
                    className="input"
                    value={timeDraft.log}
                    onChange={(e) => setTimeDraft((t) => ({ ...t, log: e.target.value }))}
                    placeholder="minutes"
                  />
                  <button
                    type="button"
                    className="btn-secondary !py-2"
                    disabled={timeDraft.log === ""}
                    onClick={() => void logTime()}
                  >
                    Log
                  </button>
                </div>
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

function fmtMins(m: number | null | undefined): string {
  if (m == null) return "–";
  const h = Math.floor(m / 60);
  const mm = m % 60;
  if (h === 0) return `${mm}m`;
  return mm === 0 ? `${h}h` : `${h}h ${mm}m`;
}

function fmtBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / (1024 * 1024)).toFixed(1)} MB`;
}

const MAX_UPLOAD_BYTES = 8 * 1024 * 1024;

function AttachmentsPanel({
  issueId,
  attachments,
  onChanged,
  onError,
}: {
  issueId: number;
  attachments: Attachment[];
  onChanged: (updater: (prev: Attachment[]) => Attachment[]) => void;
  onError: (msg: string) => void;
}) {
  const [busy, setBusy] = useState(false);

  async function upload(file: File) {
    if (!file) return;
    if (file.size > MAX_UPLOAD_BYTES) {
      onError(`File too large (max 8 MB)`);
      return;
    }
    setBusy(true);
    try {
      const data = await readFileAsBase64(file);
      const r = await api<{ data: Attachment }>(`/api/v1/issues/${issueId}/attachments`, {
        method: "POST",
        json: {
          filename: file.name,
          mime: file.type || "application/octet-stream",
          data_base64: data,
        },
      });
      onChanged((prev) => [r.data, ...prev]);
    } catch (e) {
      onError(e instanceof ApiError ? e.message : "Upload failed");
    } finally {
      setBusy(false);
    }
  }

  async function download(att: Attachment) {
    try {
      const r = await api<{
        id: number;
        filename: string;
        mime_type: string;
        size_bytes: number;
        data_base64: string;
      }>(`/api/v1/attachments/${att.id}/content`);
      const bytes = Uint8Array.from(atob(r.data_base64), (c) => c.charCodeAt(0));
      const blob = new Blob([bytes], { type: r.mime_type });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = r.filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    } catch (e) {
      onError(e instanceof ApiError ? e.message : "Download failed");
    }
  }

  async function remove(att: Attachment) {
    if (!confirm(`Delete attachment "${att.filename}"?`)) return;
    try {
      await api(`/api/v1/attachments/${att.id}`, { method: "DELETE" });
      onChanged((prev) => prev.filter((a) => a.id !== att.id));
    } catch (e) {
      onError(e instanceof ApiError ? e.message : "Delete failed");
    }
  }

  return (
    <div className="panel p-4">
      <h3 className="mb-3 text-xs font-semibold uppercase tracking-widest text-jira-muted">
        Attachments ({attachments.length})
      </h3>

      {attachments.length > 0 ? (
        <ul className="mb-3 space-y-1.5">
          {attachments.map((att) => (
            <li
              key={att.id}
              className="flex items-center gap-2 rounded-md border border-jira-border bg-jira-bg px-3 py-2"
            >
              <span className="h-2 w-2 shrink-0 rounded-full bg-jira-blue/70" />
              <button
                className="truncate text-sm text-jira-text hover:underline"
                onClick={() => void download(att)}
                title="Download"
              >
                {att.filename}
              </button>
              <span className="ml-auto shrink-0 text-xs text-jira-faint">
                {fmtBytes(att.size_bytes)}
              </span>
              <span className="shrink-0 text-[11px] text-jira-faint">{att.mime_type}</span>
              <button
                className="shrink-0 text-xs text-jira-faint transition hover:text-red-400"
                onClick={() => void remove(att)}
              >
                Delete
              </button>
            </li>
          ))}
        </ul>
      ) : (
        <p className="mb-3 text-xs text-jira-faint">No attachments.</p>
      )}

      <label className="btn-secondary inline-flex cursor-pointer !text-xs">
        {busy ? "Uploading…" : "Upload file"}
        <input
          type="file"
          className="hidden"
          disabled={busy}
          onChange={(e) => {
            const f = e.target.files?.[0];
            if (f) void upload(f);
            e.target.value = "";
          }}
        />
      </label>
    </div>
  );
}

function readFileAsBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result as string;
      const idx = result.indexOf(",");
      resolve(idx >= 0 ? result.slice(idx + 1) : result);
    };
    reader.onerror = () => reject(reader.error ?? new Error("Read failed"));
    reader.readAsDataURL(file);
  });
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