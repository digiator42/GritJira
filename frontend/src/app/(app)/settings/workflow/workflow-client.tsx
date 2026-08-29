"use client";

import { useCallback, useEffect, useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { WorkflowStep } from "@/lib/types";
import { EmptyState, ErrorBox, Field } from "@/components/ui";
import { PageShimmer } from "@/components/PageShimmer";
import { useRouter } from "next/navigation";

export function WorkflowSettingsClient() {
  const router = useRouter();
  const { currentProject } = useApp();
  const projectId = currentProject?.id;

  const [steps, setSteps] = useState<WorkflowStep[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const load = useCallback(() => {
    if (!projectId) return;
    setLoading(true);
    api<{ data: WorkflowStep[] }>(`/api/v1/projects/${projectId}/workflow`)
      .then((r) => setSteps(r.data))
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load workflow"))
      .finally(() => setLoading(false));
  }, [projectId]);

  useEffect(() => {
    load();
  }, [load]);

  async function addStep() {
    setError(null);
    setBusy(true);
    try {
      await api<WorkflowStep>(`/api/v1/projects/${projectId}/workflow/steps`, {
        method: "POST",
        json: {},
      });
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Failed to add step");
    } finally {
      setBusy(false);
    }
  }

  async function ensureDefaults() {
    if (!confirm("Create the default workflow (To Do / In Progress / In Review / Done)?")) return;
    setBusy(true);
    try {
      const r = await api<{ data: WorkflowStep[] }>(
        `/api/v1/projects/${projectId}/workflow/default`,
        { method: "POST" },
      );
      setSteps(r.data);
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Failed to create defaults");
    } finally {
      setBusy(false);
    }
  }

  async function run(action: string, stepId: number, extra?: unknown) {
    try {
      if (action === "rename") {
        await api(`/api/v1/projects/${projectId}/workflow/${stepId}`, {
          method: "PATCH",
          json: extra,
        });
      } else if (action === "toggle") {
        await api(`/api/v1/projects/${projectId}/workflow/${stepId}/toggle`, { method: "POST" });
      } else if (action === "delete") {
        await api(`/api/v1/projects/${projectId}/workflow/${stepId}`, { method: "DELETE" });
      }
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Operation failed");
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
    <div className="mx-auto max-w-2xl p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Workflow</h1>
          <p className="text-xs text-jira-muted">
            Statuses for {currentProject?.name} · ordered {steps.length > 0 ? `${steps[0]?.position}→${steps[steps.length - 1]?.position}` : "—"}
          </p>
        </div>
        <div className="flex gap-2">
          {steps.length === 0 ? (
            <button className="btn-primary" disabled={busy} onClick={() => void ensureDefaults()}>
              Create default workflow
            </button>
          ) : (
            <>
              <button className="btn-secondary" onClick={() => router.push("/board")}>
                Open board
              </button>
              <button className="btn-primary" disabled={busy} onClick={() => void addStep()}>
                + Add step
              </button>
            </>
          )}
        </div>
      </div>

      {error ? (
        <div className="mb-4">
          <ErrorBox message={error} />
        </div>
      ) : null}
      {loading && steps.length === 0 ? <PageShimmer /> : null}

      {steps.length === 0 && !loading ? (
        <EmptyState
          title="No workflow steps"
          hint="The board is driven by these columns. Create the defaults or add steps one by one."
        />
      ) : (
        <div className="panel overflow-hidden">
          <table className="w-full">
            <thead className="bg-jira-bg/60">
              <tr>
                <th className="th w-12">Order</th>
                <th className="th">Name</th>
                <th className="th">Done?</th>
                <th className="th text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-jira-border/60">
              {steps.map((step) => (
                <StepRow
                  key={step.id}
                  step={step}
                  onRename={(name) => void run("rename", step.id, { name })}
                  onToggle={() => void run("toggle", step.id)}
                  onDelete={() => {
                    if (confirm(`Delete workflow step "${step.name}"?`)) void run("delete", step.id);
                  }}
                />
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

function StepRow({
  step,
  onRename,
  onToggle,
  onDelete,
}: {
  step: WorkflowStep;
  onRename: (name: string) => void;
  onToggle: () => void;
  onDelete: () => void;
}) {
  const [editing, setEditing] = useState(false);
  const [name, setName] = useState(step.name);

  return (
    <tr className={step.is_completed ? "bg-emerald-950/10" : ""}>
      <td className="td text-jira-faint">{step.position}</td>
      <td className="td">
        {editing ? (
          <input
            autoFocus
            className="input !py-1 !text-xs"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onBlur={() => {
              setEditing(false);
              if (name.trim() && name.trim() !== step.name) onRename(name.trim());
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter") (e.target as HTMLInputElement).blur();
            }}
          />
        ) : (
          <button onClick={() => setEditing(true)} className="text-sm text-jira-text hover:underline">
            {step.name}
          </button>
        )}
      </td>
      <td className="td">
        <button
          onClick={onToggle}
          className={`rounded border px-1.5 py-0.5 text-[10px] uppercase transition ${
            step.is_completed
              ? "border-emerald-500/40 bg-emerald-500/15 text-emerald-300"
              : "border-jira-border bg-jira-bg text-jira-faint hover:text-jira-text"
          }`}
        >
          {step.is_completed ? "Done" : "To do"}
        </button>
      </td>
      <td className="td text-right">
        <button className="text-xs text-red-400/80 transition hover:text-red-300" onClick={onDelete}>
          Delete
        </button>
      </td>
    </tr>
  );
}