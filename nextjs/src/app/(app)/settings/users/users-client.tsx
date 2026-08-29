"use client";

import { useCallback, useEffect, useState } from "react";
import { api, ApiError } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { ProjectMember } from "@/lib/types";
import { Avatar, EmptyState, ErrorBox, Field, Spinner } from "@/components/ui";
import { formatDate } from "@/lib/format";
import { MEMBER_ROLES } from "@/lib/types";

export function UsersSettingsClient() {
  const { me, currentProject, users } = useApp();
  const projectId = currentProject?.id;
  const isAdmin = me?.role === "Admin";

  const [members, setMembers] = useState<ProjectMember[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(() => {
    if (!projectId) return;
    setLoading(true);
    api<{ data: ProjectMember[] }>(`/api/v1/projects/${projectId}/members`)
      .then((r) => setMembers(r.data))
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load members"))
      .finally(() => setLoading(false));
  }, [projectId]);

  useEffect(() => {
    load();
  }, [load]);

  async function changeRole(memberId: number, role: string) {
    try {
      await api(`/api/v1/projects/${projectId}/members/${memberId}`, {
        method: "PATCH",
        json: { user_id: memberId, role },
      });
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Role update failed");
    }
  }

  async function remove(member: ProjectMember) {
    if (!confirm(`Remove ${member.username} from ${currentProject?.name}?`)) return;
    try {
      await api(`/api/v1/projects/${projectId}/members/${member.id}`, { method: "DELETE" });
      load();
    } catch (e) {
      setError(e instanceof ApiError ? e.message : "Remove failed");
    }
  }

  if (!projectId) {
    return (
      <div className="p-4">
        <ErrorBox message="No project selected. Pick a project from the sidebar." />
      </div>
    );
  }

  const unassigned = users.filter(
    (u) => !members.some((m) => m.user_id === u.id),
  );

  return (
    <div className="mx-auto max-w-3xl p-4">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-base font-semibold text-jira-text">Users & Members</h1>
          <p className="text-xs text-jira-muted">
            Members of {currentProject?.name} ({members.length})
          </p>
        </div>
      </div>

      {error ? <ErrorBox message={error} /> : null}
      {loading && members.length === 0 ? <Spinner label="Loading members…" /> : null}

      {isAdmin ? (
        <AddMember users={unassigned} projectId={projectId} onAdded={load} />
      ) : (
        <div className="panel p-3 text-xs text-jira-faint">
          You have read-only access to this project&apos;s members. Only project admins can add
          members or change roles.
        </div>
      )}

      <section className="panel mt-4 overflow-hidden">
        {members.length === 0 ? (
          <EmptyState title="No members yet" hint="Add a user to this project." />
        ) : (
          <table className="w-full">
            <thead className="bg-jira-bg/60">
              <tr>
                <th className="th">User</th>
                <th className="th">Role</th>
                <th className="th">Joined</th>
                {isAdmin ? <th className="th text-right">Actions</th> : null}
              </tr>
            </thead>
            <tbody className="divide-y divide-jira-border/60">
              {members.map((m) => (
                <tr key={m.id}>
                  <td className="td">
                    <div className="flex items-center gap-2">
                      <Avatar name={m.username} size={24} />
                      <span>{m.username}</span>
                    </div>
                  </td>
                  <td className="td">
                    {isAdmin ? (
                      <select
                        className="input !w-auto !py-1 !text-xs"
                        value={m.role}
                        onChange={(e) => void changeRole(m.id, e.target.value)}
                      >
                        {MEMBER_ROLES.map((r) => (
                          <option key={r} value={r}>
                            {r}
                          </option>
                        ))}
                      </select>
                    ) : (
                      <span className="text-xs capitalize text-jira-muted">{m.role}</span>
                    )}
                  </td>
                  <td className="td text-jira-faint">{formatDate(m.joined_at)}</td>
                  {isAdmin ? (
                    <td className="td text-right">
                      <button
                        className="text-xs text-jira-faint transition hover:text-red-400"
                        onClick={() => void remove(m)}
                      >
                        Remove
                      </button>
                    </td>
                  ) : null}
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </section>
    </div>
  );
}

function AddMember({
  users,
  projectId,
  onAdded,
}: {
  users: { id: number; username: string }[];
  projectId: number;
  onAdded: () => void;
}) {
  const [userId, setUserId] = useState("");
  const [role, setRole] = useState("Developer");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(e: React.FormEvent) {
    e.preventDefault();
    if (!userId) return;
    setError(null);
    setBusy(true);
    try {
      await api(`/api/v1/projects/${projectId}/members`, {
        method: "POST",
        json: { user_id: Number(userId), role },
      });
      setUserId("");
      onAdded();
    } catch (err) {
      setError(err instanceof ApiError ? err.message : "Failed to add member");
    } finally {
      setBusy(false);
    }
  }

  if (users.length === 0) {
    return (
      <div className="panel p-3 text-xs text-jira-faint">
        All registered users are already members of this project.
      </div>
    );
  }

  return (
    <form onSubmit={submit} className="panel flex flex-wrap items-end gap-3 p-3">
      <Field label="User">
        <select className="input !w-56" value={userId} onChange={(e) => setUserId(e.target.value)}>
          <option value="">Select user…</option>
          {users.map((u) => (
            <option key={u.id} value={u.id}>
              {u.username}
            </option>
          ))}
        </select>
      </Field>
      <Field label="Role">
        <select className="input" value={role} onChange={(e) => setRole(e.target.value)}>
          {MEMBER_ROLES.map((r) => (
            <option key={r} value={r}>
              {r}
            </option>
          ))}
        </select>
      </Field>
      {error ? <p className="w-full text-xs text-red-300">{error}</p> : null}
      <button type="submit" disabled={busy || !userId} className="btn-primary">
        {busy ? "Adding…" : "Add member"}
      </button>
    </form>
  );
}