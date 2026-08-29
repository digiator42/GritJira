"use client";

import { useState } from "react";
import Link from "next/link";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Project } from "@/lib/types";
import { EmptyState, ErrorBox, Field, Modal } from "@/components/ui";
import { PageShimmer } from "@/components/PageShimmer";
import { useRequest } from "@/lib/hooks";
import { formatDate, decodeEntities } from "@/lib/format";

export function ProjectsClient() {
  const { selectProject } = useApp();
  const { data, error, loading, reload } = useRequest<Project[]>(
    async () => (await api<{ data: Project[] }>("/api/v1/projects")).data,
    [],
  );
  const [createOpen, setCreateOpen] = useState(false);

  async function deleteProject(id: number, name: string) {
    if (!confirm(`Delete project "${name}" and all of its issues?`)) return;
    try {
      await api(`/api/v1/projects/${id}`, { method: "DELETE" });
      reload();
    } catch (e) {
      alert(e instanceof ApiError ? e.message : "Delete failed");
    }
  }

  return (
    <div className="p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Projects</h1>
          <p className="text-xs text-jira-muted">{data?.length ?? 0} projects</p>
        </div>
        <button className="btn-primary" onClick={() => setCreateOpen(true)}>
          + Create project
        </button>
      </div>

      {error ? <ErrorBox message={error} /> : null}
      {loading ? (
        <PageShimmer />
      ) : !data || data.length === 0 ? (
        <EmptyState title="No projects yet" hint="Create your first project to get started." />
      ) : (
        <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
          {data.map((p) => (
            <div key={p.id} className="panel flex flex-col p-4">
              <div className="mb-2 flex items-center justify-between">
                <span className="rounded bg-jira-blue/20 px-2 py-0.5 text-xs font-bold text-jira-blue">
                  {p.key.toUpperCase()}
                </span>
                <button
                  className="text-xs text-jira-faint transition hover:text-red-400"
                  onClick={() => void deleteProject(p.id, p.name)}
                  title="Delete project"
                >
                  ✕
                </button>
              </div>
              <Link href={`/projects/${p.id}`} className="text-sm font-semibold text-jira-text hover:underline">
                {p.name}
              </Link>
              <p className="mt-1 line-clamp-2 flex-1 text-xs text-jira-muted">
                {decodeEntities(p.description) || "No description."}
              </p>
              <div className="mt-3 flex items-center justify-between border-t border-jira-border pt-2 text-[10px] text-jira-faint">
                <span>Created {formatDate(p.created_at)}</span>
                <button
                  className="text-jira-blue hover:underline"
                  onClick={() => {
                    selectProject(p.id);
                    window.location.href = "/board";
                  }}
                >
                  Open board →
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      <Modal open={createOpen} onClose={() => setCreateOpen(false)} title="Create project">
        <ProjectCreateForm
          onCreated={() => {
            setCreateOpen(false);
            reload();
          }}
        />
      </Modal>
    </div>
  );
}

function ProjectCreateForm({ onCreated }: { onCreated: () => void }) {
  const [key, setKey] = useState("");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setBusy(true);
    try {
      await api<Project>("/api/v1/projects", {
        method: "POST",
        json: { key, name, description: description || undefined },
      });
      onCreated();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to create project");
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit} className="space-y-3">
      <div className="grid grid-cols-3 gap-3">
        <Field label="Key">
          <input
            required
            maxLength={10}
            className="input uppercase"
            value={key}
            onChange={(e) => setKey(e.target.value.toUpperCase())}
            placeholder="GRT"
          />
        </Field>
        <div className="col-span-2">
          <Field label="Name">
            <input
              required
              className="input"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Grit Project"
            />
          </Field>
        </div>
      </div>
      <Field label="Description">
        <textarea
          className="input min-h-[70px]"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Optional…"
        />
      </Field>
      {error ? <p className="text-xs text-red-300">{error}</p> : null}
      <div className="flex justify-end gap-2">
        <button type="button" className="btn-secondary" onClick={onCreated}>
          Cancel
        </button>
        <button type="submit" disabled={busy} className="btn-primary">
          {busy ? "Creating…" : "Create project"}
        </button>
      </div>
    </form>
  );
}