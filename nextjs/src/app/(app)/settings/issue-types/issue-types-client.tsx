"use client";

import { useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { IssueType } from "@/lib/types";
import { EmptyState, ErrorBox, Field } from "@/components/ui";
import { IssueTypeIcon, ISSUE_TYPE_ICON_KEYS } from "@/components/IssueTypeIcon";

const COLOR_PALETTE = [
  "#eb5a46",
  "#65ba43",
  "#4bade9",
  "#a25dd8",
  "#8c9bab",
  "#ff8b45",
  "#2daeb7",
  "#f6b93b",
  "#d9764f",
  "#a0a4b8",
];

export function IssueTypesSettingsClient() {
  const { currentProject, issueTypes, refreshIssueTypes } = useApp();
  const [error, setError] = useState<string | null>(null);
  const [editing, setEditing] = useState<IssueType | null>(null);

  if (!currentProject) {
    return (
      <div className="p-4">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }
  const projectId = currentProject.id;

  async function remove(t: IssueType) {
    if (!confirm(`Delete issue type "${t.name}"?`)) return;
    setError(null);
    try {
      await api(`/api/v1/projects/${projectId}/issue-types/${t.id}`, {
        method: "DELETE",
      });
      await refreshIssueTypes();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Delete failed");
    }
  }

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Issue types</h1>
          <p className="text-xs text-jira-muted">
            Custom types available when creating issues in {currentProject.key}.
          </p>
        </div>
      </div>

      {error ? (
        <div className="mb-4">
          <ErrorBox message={error} />
        </div>
      ) : null}

      <CreateIssueTypeForm projectId={projectId} onSaved={refreshIssueTypes} />

      <section className="panel mt-4 overflow-hidden">
        {issueTypes.length === 0 ? (
          <EmptyState title="No issue types yet" hint="Add one below." />
        ) : (
          <table className="w-full">
            <thead className="bg-jira-bg/60">
              <tr>
                <th className="th">Icon</th>
                <th className="th">Name</th>
                <th className="th">Key</th>
                <th className="th">Color</th>
                <th className="th text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-jira-border/60">
              {issueTypes.map((t) => (
                <tr key={t.id}>
                  <td className="td">
                    <IssueTypeIcon iconKey={t.icon_key} color={t.color} size={18} title={t.name} />
                  </td>
                  <td className="td font-medium capitalize">{t.name}</td>
                  <td className="td">
                    <code className="rounded bg-jira-bg px-1.5 py-0.5 text-xs text-jira-accent">
                      {t.icon_key}
                    </code>
                  </td>
                  <td className="td">
                    <span className="inline-flex items-center gap-1.5 text-xs text-jira-muted">
                      <span className="size-3 rounded-sm" style={{ backgroundColor: t.color }} />
                      {t.color}
                    </span>
                  </td>
                  <td className="td text-right">
                    <button
                      className="text-xs text-jira-faint transition hover:text-jira-blue"
                      onClick={() => setEditing(t)}
                    >
                      Edit
                    </button>
                    <button
                      className="ml-3 text-xs text-jira-faint transition hover:text-red-400"
                      onClick={() => void remove(t)}
                    >
                      Delete
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </section>

      {editing ? (
        <EditIssueTypeForm
          issueType={editing}
          projectId={projectId}
          onDone={() => {
            setEditing(null);
            void refreshIssueTypes();
          }}
        />
      ) : null}
    </div>
  );
}

function TypeFormFields({
  name,
  setName,
  iconKey,
  setIconKey,
  color,
  setColor,
}: {
  name: string;
  setName: (v: string) => void;
  iconKey: string;
  setIconKey: (v: string) => void;
  color: string;
  setColor: (v: string) => void;
}) {
  return (
    <>
      <Field label="Name">
        <input
          required
          className="input"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="e.g. Spike"
        />
      </Field>
      <Field label="Icon">
        <div className="flex flex-wrap gap-1.5">
          {ISSUE_TYPE_ICON_KEYS.map((k) => (
            <button
              key={k}
              type="button"
              className={`rounded-lg border p-1.5 transition ${
                iconKey === k
                  ? "border-jira-blue bg-jira-blue/15"
                  : "border-jira-border hover:border-jira-blue/50"
              }`}
              onClick={() => setIconKey(k)}
            >
              <IssueTypeIcon iconKey={k} color={color} size={18} />
            </button>
          ))}
        </div>
      </Field>
      <Field label="Color">
        <div className="flex flex-wrap gap-1.5">
          {COLOR_PALETTE.map((c) => (
            <button
              key={c}
              type="button"
              className={`size-6 rounded-full border-2 transition ${
                color === c ? "border-white" : "border-transparent hover:border-jira-border"
              }`}
              style={{ backgroundColor: c }}
              onClick={() => setColor(c)}
            />
          ))}
          <span className="ml-2 inline-flex items-center gap-1.5 text-xs text-jira-muted">
            <IssueTypeIcon iconKey={iconKey} color={color} size={16} />
            Preview
          </span>
        </div>
      </Field>
    </>
  );
}

function CreateIssueTypeForm({
  projectId,
  onSaved,
}: {
  projectId: number;
  onSaved: () => void;
}) {
  const [name, setName] = useState("");
  const [iconKey, setIconKey] = useState("task");
  const [color, setColor] = useState(COLOR_PALETTE[2]);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim()) return;
    setError(null);
    setBusy(true);
    try {
      await api(`/api/v1/projects/${projectId}/issue-types`, {
        method: "POST",
        json: { name: name.trim(), icon_key: iconKey, color },
      });
      setName("");
      setIconKey("task");
      setColor(COLOR_PALETTE[2]);
      onSaved();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to create issue type");
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit} className="panel p-3">
      <p className="mb-3 text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
        New issue type
      </p>
      <div className="flex flex-wrap items-end gap-3">
        <TypeFormFields
          name={name}
          setName={setName}
          iconKey={iconKey}
          setIconKey={setIconKey}
          color={color}
          setColor={setColor}
        />
        <button type="submit" disabled={busy || !name.trim()} className="btn-primary">
          {busy ? "Creating…" : "Create"}
        </button>
      </div>
      {error ? <p className="mt-2 text-xs text-red-300">{error}</p> : null}
    </form>
  );
}

function EditIssueTypeForm({
  issueType,
  projectId,
  onDone,
}: {
  issueType: IssueType;
  projectId: number;
  onDone: () => void;
}) {
  const [name, setName] = useState(issueType.name);
  const [iconKey, setIconKey] = useState(issueType.icon_key);
  const [color, setColor] = useState(issueType.color);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim()) return;
    setError(null);
    setBusy(true);
    try {
      await api(`/api/v1/projects/${projectId}/issue-types/${issueType.id}`, {
        method: "PATCH",
        json: { name: name.trim(), icon_key: iconKey, color },
      });
      onDone();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to update issue type");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-black/60" onClick={onDone} aria-hidden />
      <div className="relative w-full max-w-lg rounded-md border border-jira-border bg-jira-panel p-4 shadow-2xl">
        <div className="mb-3 flex items-center justify-between">
          <h2 className="text-sm font-semibold text-jira-text">Edit issue type</h2>
          <button className="text-xs text-jira-faint hover:text-jira-text" onClick={onDone}>
            ✕
          </button>
        </div>
        <form onSubmit={submit} className="space-y-3">
          <TypeFormFields
            name={name}
            setName={setName}
            iconKey={iconKey}
            setIconKey={setIconKey}
            color={color}
            setColor={setColor}
          />
          {error ? <p className="text-xs text-red-300">{error}</p> : null}
          <div className="flex justify-end gap-2 pt-1">
            <button type="button" className="btn-secondary" onClick={onDone}>
              Cancel
            </button>
            <button type="submit" disabled={busy} className="btn-primary">
              {busy ? "Saving…" : "Save"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
