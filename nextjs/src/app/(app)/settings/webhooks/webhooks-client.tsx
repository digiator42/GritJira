"use client";

import { useCallback, useEffect, useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Webhook } from "@/lib/types";
import { WEBHOOK_EVENTS } from "@/lib/types";
import { EmptyState, ErrorBox, Field } from "@/components/ui";
import { PageShimmer } from "@/components/PageShimmer";
import { formatDate } from "@/lib/format";

export function WebhooksSettingsClient() {
  const { currentProject } = useApp();
  const projectId = currentProject?.id;

  const [hooks, setHooks] = useState<Webhook[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(() => {
    if (!projectId) return;
    setLoading(true);
    api<{ data: Webhook[] }>(`/api/v1/webhooks?project_id=${projectId}`)
      .then((r) => setHooks(r.data))
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load webhooks"))
      .finally(() => setLoading(false));
  }, [projectId]);

  useEffect(() => {
    load();
  }, [load]);

  async function remove(hook: Webhook) {
    if (!confirm(`Delete webhook "${hook.name}"?`)) return;
    try {
      await api(`/api/v1/webhooks/${hook.id}`, { method: "DELETE" });
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Delete failed");
    }
  }

  if (!projectId) {
    return (
      <div className="p-4">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Webhooks</h1>
          <p className="text-xs text-jira-muted">
            POST JSON event notifications to external endpoints whenever issues change.
          </p>
        </div>
      </div>

      {error ? <ErrorBox message={error} /> : null}
      {loading && hooks.length === 0 ? <PageShimmer /> : null}

      <AddWebhook projectId={projectId} onAdded={load} />

      <section className="panel mt-4 overflow-hidden">
        {hooks.length === 0 ? (
          <EmptyState title="No webhooks yet" hint="Create one to receive event notifications." />
        ) : (
          <table className="w-full">
            <thead className="bg-jira-bg/60">
              <tr>
                <th className="th">Name</th>
                <th className="th">Event</th>
                <th className="th">URL</th>
                <th className="th">Created</th>
                <th className="th text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-jira-border/60">
              {hooks.map((h) => (
                <tr key={h.id}>
                  <td className="td">
                    <div className="flex items-center gap-2">
                      <span
                        className={`size-1.5 rounded-full ${h.is_active ? "bg-emerald-400" : "bg-jira-faint"}`}
                      />
                      <span className="font-medium">{h.name}</span>
                    </div>
                  </td>
                  <td className="td">
                    <code className="rounded bg-jira-bg px-1.5 py-0.5 text-xs text-jira-accent">
                      {h.event}
                    </code>
                  </td>
                  <td className="td break-all font-mono text-xs text-jira-muted">{h.url}</td>
                  <td className="td text-jira-faint">{formatDate(h.created_at)}</td>
                  <td className="td text-right">
                    <button
                      className="text-xs text-jira-faint transition hover:text-red-400"
                      onClick={() => void remove(h)}
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
    </div>
  );
}

function AddWebhook({
  projectId,
  onAdded,
}: {
  projectId: number;
  onAdded: () => void;
}) {
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");
  const [event, setEvent] = useState("issue.moved");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim() || !url.trim()) return;
    setError(null);
    setBusy(true);
    try {
      await api(`/api/v1/webhooks?project_id=${projectId}`, {
        method: "POST",
        json: { name: name.trim(), url: url.trim(), event },
      });
      setName("");
      setUrl("");
      setEvent("issue.moved");
      onAdded();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to create webhook");
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={submit} className="panel flex flex-wrap items-end gap-3 p-3">
      <Field label="Name">
        <input
          className="input !w-40"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="e.g. Slack bot"
        />
      </Field>
      <Field label="Event">
        <select className="input" value={event} onChange={(e) => setEvent(e.target.value)}>
          {WEBHOOK_EVENTS.map((ev) => (
            <option key={ev.value} value={ev.value}>
              {ev.label}
            </option>
          ))}
        </select>
      </Field>
      <Field label="URL">
        <input
          className="input !w-72 font-mono !text-xs"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="http://host:port/path"
        />
      </Field>
      {error ? <p className="w-full text-xs text-red-300">{error}</p> : null}
      <button type="submit" disabled={busy || !name.trim() || !url.trim()} className="btn-primary">
        {busy ? "Creating…" : "Create webhook"}
      </button>
    </form>
  );
}