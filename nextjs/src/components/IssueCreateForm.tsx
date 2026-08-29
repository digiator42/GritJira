"use client";

import { useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import { PRIORITIES, DEFAULT_ISSUE_TYPE_STYLES, type IssueType, type User } from "@/lib/types";
import { Avatar, Field } from "./ui";
import { Dropdown } from "./Dropdown";
import { IssueTypeIcon, resolveIssueType } from "./IssueTypeIcon";

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
          <TypePicker value={issueType} onChange={setIssueType} options={issueTypes} />
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
          <AssigneePicker users={users} value={assigneeId} onChange={setAssigneeId} />
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

function TypePicker({
  value,
  onChange,
  options,
}: {
  value: string;
  onChange: (v: string) => void;
  options: IssueType[];
}) {
  const names =
    options.length > 0 ? options.map((t) => t.name) : Object.keys(DEFAULT_ISSUE_TYPE_STYLES);
  const selected = resolveIssueType(value, options);
  return (
    <Dropdown
      align="left"
      panelClassName="w-full"
      trigger={({ open, toggle }) => (
        <button
          type="button"
          onClick={toggle}
          className={`input flex items-center justify-between gap-2 !text-left ${
            open ? "!border-jira-blue" : ""
          }`}
        >
          <span className="inline-flex min-w-0 items-center gap-2">
            <IssueTypeIcon iconKey={selected.icon_key} color={selected.color} size={16} />
            <span className="truncate text-sm capitalize text-jira-text">{selected.label}</span>
          </span>
          <Chevron open={open} />
        </button>
      )}
    >
      <div className="max-h-60 overflow-auto py-1">
        {names.map((name) => {
          const s = resolveIssueType(name, options);
          const active = value.toLowerCase() === name.toLowerCase();
          return (
            <button
              key={name}
              type="button"
              onClick={() => onChange(name)}
              className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
                active
                  ? "bg-jira-blue/15 font-medium text-jira-text"
                  : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
              }`}
            >
              <IssueTypeIcon iconKey={s.icon_key} color={s.color} size={16} />
              <span className="flex-1 truncate text-left capitalize">{s.label}</span>
              {active ? (
                <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
                  <path
                    d="m5 13 4 4L19 7"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  />
                </svg>
              ) : null}
            </button>
          );
        })}
      </div>
    </Dropdown>
  );
}

function AssigneePicker({
  users,
  value,
  onChange,
}: {
  users: User[];
  value: string;
  onChange: (v: string) => void;
}) {
  const selected = users.find((u) => String(u.id) === value);
  return (
    <Dropdown
      align="left"
      panelClassName="w-full"
      trigger={({ open, toggle }) => (
        <button
          type="button"
          onClick={toggle}
          className={`input flex items-center justify-between gap-2 !text-left ${
            open ? "!border-jira-blue" : ""
          }`}
        >
          <span className="inline-flex min-w-0 items-center gap-2">
            {selected ? (
              <Avatar name={selected.username} size={20} />
            ) : (
              <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-jira-border/60 text-[10px] text-jira-faint">
                ?
              </span>
            )}
            <span className="truncate text-sm text-jira-text">
              {selected ? selected.username : "Unassigned"}
            </span>
          </span>
          <Chevron open={open} />
        </button>
      )}
    >
      <div className="max-h-56 overflow-auto py-1">
        <button
          type="button"
          onClick={() => onChange("")}
          className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
            value === ""
              ? "bg-jira-blue/15 font-medium text-jira-text"
              : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
          }`}
        >
          <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-jira-border/60 text-[10px] text-jira-faint">
            ?
          </span>
          <span className="flex-1 truncate text-left text-jira-muted">Unassigned</span>
          {value === "" ? (
            <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
              <path
                d="m5 13 4 4L19 7"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
          ) : null}
        </button>
        {users.map((u) => {
          const active = String(u.id) === value;
          return (
            <button
              key={u.id}
              type="button"
              onClick={() => onChange(String(u.id))}
              className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
                active
                  ? "bg-jira-blue/15 font-medium text-jira-text"
                  : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
              }`}
            >
              <Avatar name={u.username} size={20} />
              <span className="flex-1 truncate text-left">{u.username}</span>
              <span className="shrink-0 text-[10px] uppercase tracking-wide text-jira-faint">
                {u.role}
              </span>
              {active ? (
                <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
                  <path
                    d="m5 13 4 4L19 7"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  />
                </svg>
              ) : null}
            </button>
          );
        })}
      </div>
    </Dropdown>
  );
}

function Chevron({ open }: { open: boolean }) {
  return (
    <svg
      viewBox="0 0 24 24"
      className={`h-4 w-4 shrink-0 text-jira-faint transition-transform ${open ? "rotate-180" : ""}`}
      fill="none"
    >
      <path d="m6 9 6 6 6-6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}