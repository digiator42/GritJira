"use client";

import { useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import { PRIORITIES } from "@/lib/types";
import { Field } from "./ui";

export function IssueCreateForm({
  projectId,
  sprintId,
  onCreated,
}: {
  projectId: number;
  sprintId?: number;
  onCreated: () => void;
}) {
  const { users, issueTypes } = useApp();
  const [summary, setSummary] = useState("");
  const [description, setDescription] = useState("");
  const [issueType, setIssueType] = useState("story");
  const [priority, setPriority] = useState("3");
  const [storyPoints, setStoryPoints] = useState("");
  const [estimate, setEstimate] = useState("");
  const [dueDate, setDueDate] = useState("");
  const [assigneeId, setAssigneeId] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setBusy(true);
    try {
      await api(`/api/v1/issues?project_id=${projectId}`, {
        method: "POST",
        json: {
          summary,
          description: description || undefined,
          issue_type: issueType,
          priority: Number(priority),
          sprint_id: sprintId ?? null,
          story_points: storyPoints ? Number(storyPoints) : null,
          time_estimate_minutes: estimate ? Number(estimate) : null,
          due_date: dueDate || null,
          assignee_id: assigneeId ? Number(assigneeId) : null,
        },
      });
      onCreated();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to create issue");
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit} className="space-y-3">
      <Field label="Summary">
        <input
          required
          className="input"
          value={summary}
          onChange={(e) => setSummary(e.target.value)}
          placeholder="What needs to be done?"
        />
      </Field>

      <Field label="Description">
        <textarea
          className="input min-h-[80px] resize-y"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Optional details…"
        />
      </Field>

      <div className="grid grid-cols-2 gap-3">
        <Field label="Type">
          <select className="input" value={issueType} onChange={(e) => setIssueType(e.target.value)}>
            {(issueTypes.length > 0 ? issueTypes : [{ id: 0, name: "story" }, { id: 0, name: "bug" }, { id: 0, name: "task" }]).map(
              (t) => (
                <option key={`${t.id}-${t.name}`} value={t.name}>
                  {t.name}
                </option>
              ),
            )}
          </select>
        </Field>
        <Field label="Priority">
          <select
            className="input"
            value={priority}
            onChange={(e) => setPriority(e.target.value)}
          >
            {PRIORITIES.map((p) => (
              <option key={p.value} value={p.value}>
                {p.label}
              </option>
            ))}
          </select>
        </Field>
        <Field label="Assignee">
          <select
            className="input"
            value={assigneeId}
            onChange={(e) => setAssigneeId(e.target.value)}
          >
            <option value="">Unassigned</option>
            {users.map((u) => (
              <option key={u.id} value={u.id}>
                {u.username}
              </option>
            ))}
          </select>
        </Field>
        <Field label="Due date">
          <input
            type="date"
            className="input"
            value={dueDate}
            onChange={(e) => setDueDate(e.target.value)}
          />
        </Field>
        <Field label="Story points">
          <input
            type="number"
            min={0}
            className="input"
            value={storyPoints}
            onChange={(e) => setStoryPoints(e.target.value)}
            placeholder="e.g. 3"
          />
        </Field>
        <Field label="Estimate (min)">
          <input
            type="number"
            min={0}
            className="input"
            value={estimate}
            onChange={(e) => setEstimate(e.target.value)}
            placeholder="e.g. 240"
          />
        </Field>
      </div>

      {error ? (
        <p className="rounded-md border border-red-900/50 bg-red-950/20 px-3 py-2 text-xs text-red-300">
          {error}
        </p>
      ) : null}

      <div className="flex justify-end gap-2 pt-1">
        <button type="button" className="btn-secondary" onClick={onCreated}>
          Cancel
        </button>
        <button type="submit" disabled={busy} className="btn-primary">
          {busy ? "Creating…" : "Create issue"}
        </button>
      </div>
    </form>
  );
}