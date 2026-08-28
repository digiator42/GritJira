"use client";

import { api } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Issue, Project } from "@/lib/types";
import { EmptyState, ErrorBox, Spinner } from "@/components/ui";
import { useRequest } from "@/lib/hooks";
import { useRouter } from "next/navigation";
import { formatDate, userById, decodeEntities } from "@/lib/format";

interface ProjectIssues {
  project: Project & { relations?: Record<string, unknown> };
  issues: Issue[];
}

export function ProjectDetailClient({ id }: { id: number }) {
  const router = useRouter();
  const { users, selectProject } = useApp();
  const { data, error, loading } = useRequest<ProjectIssues>(
    async () => (await api<{ data: ProjectIssues }>(`/api/v1/projects/${id}/issues`)).data,
    [id],
  );

  const project = data?.project;

  return (
    <div className="p-4">
      {loading && !data ? <Spinner label="Loading project…" /> : null}
      {error ? <ErrorBox message={error} /> : null}
      {data && project ? (
        <>
          <div className="mb-4 panel p-4">
            <div className="mb-2 flex items-center justify-between gap-3">
              <div className="flex items-center gap-3">
                <span className="rounded bg-jira-blue/20 px-2 py-1 text-sm font-bold text-jira-blue">
                  {project.key.toUpperCase()}
                </span>
                <h1 className="text-lg font-semibold text-jira-text">{project.name}</h1>
              </div>
              <div className="flex gap-2">
                <button
                  className="btn-secondary"
                  onClick={() => {
                    selectProject(id);
                    router.push("/board");
                  }}
                >
                  Open board
                </button>
                <button className="btn-secondary" onClick={() => router.push("/backlog")}>
                  Backlog
                </button>
              </div>
            </div>
            <p className="text-sm text-jira-muted">
              {decodeEntities(project.description) || "No description."}
            </p>
            <p className="mt-2 text-[10px] text-jira-faint">
              Created {formatDate(project.created_at)}
            </p>
          </div>

          <section className="panel overflow-hidden">
            <div className="border-b border-jira-border px-4 py-2 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Issues ({data.issues.length})
            </div>
            {data.issues.length === 0 ? (
              <EmptyState title="No issues in this project yet" />
            ) : (
              <table className="w-full">
                <thead className="bg-jira-bg/60">
                  <tr>
                    <th className="th">Key</th>
                    <th className="th">Summary</th>
                    <th className="th">Type</th>
                    <th className="th">Assignee</th>
                    <th className="th">Story points</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-jira-border/60">
                  {data.issues.map((issue) => (
                    <tr
                      key={issue.id}
                      className="cursor-pointer transition hover:bg-jira-border/30"
                      onClick={() => router.push(`/issues/${issue.id}`)}
                    >
                      <td className="td text-jira-faint">{issue.key}</td>
                      <td className="td text-jira-text">{decodeEntities(issue.summary)}</td>
                      <td className="td capitalize text-jira-muted">{issue.issue_type}</td>
                      <td className="td text-jira-muted">{userById(users, issue.assignee_id)}</td>
                      <td className="td text-jira-faint">{issue.story_points ?? "—"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}