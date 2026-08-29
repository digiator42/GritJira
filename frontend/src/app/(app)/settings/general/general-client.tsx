"use client";

import { useEffect, useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import { ErrorBox, Field } from "@/components/ui";

export function GeneralSettingsClient() {
  const { currentProject, refreshProjects } = useApp();
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (currentProject) {
      setName(currentProject.name ?? "");
      setDescription(currentProject.description ?? "");
    }
  }, [currentProject?.id]);

  if (!currentProject) {
    return (
      <div className="p-4">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }
  const projectId = currentProject.id;

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setSaved(false);
    setBusy(true);
    try {
      await api(`/api/v1/projects/${projectId}`, {
        method: "PATCH",
        json: {
          name: name.trim() || undefined,
          description: description.trim() || null,
        },
      });
      await refreshProjects();
      setSaved(true);
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to save project settings");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-4">
        <h1 className="text-base font-semibold text-jira-text">General settings</h1>
        <p className="text-xs text-jira-muted">
          Project name and description for {currentProject.key}.
        </p>
      </div>

      {error ? (
        <div className="mb-4">
          <ErrorBox message={error} />
        </div>
      ) : null}

      <form onSubmit={submit} className="panel space-y-3 p-4">
        <div className="mb-2 text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          {currentProject.key}
        </div>
        <Field label="Project name">
          <input
            required
            className="input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Project name"
          />
        </Field>
        <Field label="Description">
          <textarea
            className="input min-h-[90px] resize-y"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="What is this project about?"
          />
        </Field>
        <div className="flex items-center gap-3 pt-1">
          <button type="submit" disabled={busy} className="btn-primary">
            {busy ? "Saving…" : "Save changes"}
          </button>
          {saved ? <span className="text-xs text-emerald-400">Saved.</span> : null}
        </div>
      </form>
    </div>
  );
}