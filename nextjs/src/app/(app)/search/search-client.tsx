"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { api } from "@/lib/api";
import { useApp } from "@/lib/AppContext";
import type { Issue, Project } from "@/lib/types";
import { ErrorBox, PriorityBadge, TypeBadge } from "@/components/ui";
import { userById, decodeEntities } from "@/lib/format";

const EXAMPLES = [
  "priority = 3",
  "issue_type = bug",
  "summary LIKE deploy",
  "key LIKE GRIT",
];

export function SearchClient() {
  const router = useRouter();
  const { currentProject, users } = useApp();
  const [query, setQuery] = useState("");
  const [issues, setIssues] = useState<Issue[] | null>(null);
  const [projects, setProjects] = useState<Project[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [searched, setSearched] = useState(false);

  async function run(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setBusy(true);
    setSearched(true);

    const trimmed = query.trim();

    // The backend JQL engine accepts a single `column op value` condition.
    // Priority filter / issue scope cannot be combined, so a bare keyword
    // becomes a cross-project `summary LIKE foo` search.
    const isCondition =
      /^(summary|description|key|priority|issue_type|assignee_id|reporter_id|sprint_id|project_id)\s*(=|LIKE|like|>|<)\s*\S+$/.test(
        trimmed,
      );

    const jql = isCondition ? `${trimmed}` : /^\S+$/.test(trimmed) ? `summary LIKE ${trimmed}` : null;

    try {
      if (!jql) {
        setIssues(null);
        setError(
          "Enter a single keyword, or a condition like `priority = 3` or `summary LIKE deploy`.",
        );
        return;
      }
      const [issueRes, projRes] = await Promise.all([
        api<{ data: Issue[] }>(`/api/v1/issues/search?jql=${encodeURIComponent(jql)}`).catch(
          () => null,
        ),
        // separate plain-text project search
        api<{ data: Project[] }>(
          `/api/v1/projects/search?q=${encodeURIComponent(trimmed.replace(/\s+\S+\s*[=<>]\s*\S+\s*$/, ""))}`,
        ).catch(() => ({ data: [] })),
      ]);
      setIssues(issueRes?.data ?? null);
      setProjects(projRes?.data ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Search failed");
      setIssues([]);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="mx-auto max-w-3xl p-4">
      <h1 className="mb-1 text-base font-semibold text-jira-text">Search</h1>
      <p className="mb-4 text-xs text-jira-muted">
        Issue search is cross-project (the JQL engine combines a single condition). Results from{" "}
        {currentProject?.name ?? "project 1"} are highlighted by project.
      </p>

      <form onSubmit={run} className="mb-2 flex gap-2">
        <input
          className="input"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="e.g. priority = 3, summary LIKE deploy, key LIKE GRIT, or a keyword"
        />
        <button type="submit" disabled={busy} className="btn-primary shrink-0">
          {busy ? "…" : "Search"}
        </button>
      </form>
      <div className="mb-4 flex flex-wrap gap-1.5">
        {EXAMPLES.map((ex) => (
          <button
            key={ex}
            type="button"
            className="rounded border border-jira-border bg-jira-panel px-2 py-0.5 text-[10px] text-jira-muted transition hover:border-jira-blue/50 hover:text-jira-text"
            onClick={() => setQuery(ex)}
          >
            {ex}
          </button>
        ))}
      </div>

      {error ? <ErrorBox message={error} /> : null}

      {searched && !busy && !error ? (
        <div className="space-y-4">
          <section>
            <h2 className="mb-2 text-xs font-semibold uppercase tracking-widest text-jira-muted">
              Issues ({issues?.length ?? 0})
            </h2>
            {issues && issues.length > 0 ? (
              <div className="panel overflow-hidden">
                <table className="w-full">
                  <tbody className="divide-y divide-jira-border/60">
                    {issues.map((issue) => (
                      <tr
                        key={issue.id}
                        className="cursor-pointer transition hover:bg-jira-border/30"
                        onClick={() => router.push(`/issues/${issue.id}`)}
                      >
                        <td className="td w-28 text-jira-faint">{issue.key}</td>
                        <td className="td text-jira-text">{decodeEntities(issue.summary)}</td>
                        <td className="td w-20">
                          <TypeBadge type={issue.issue_type} />
                        </td>
                        <td className="td w-20">
                          <PriorityBadge value={issue.priority} />
                        </td>
                        <td className="td w-28 text-jira-muted">
                          {userById(users, issue.assignee_id)}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            ) : (
              <p className="text-xs text-jira-faint">No issues matched.</p>
            )}
          </section>

          {projects.length > 0 ? (
            <section>
              <h2 className="mb-2 text-xs font-semibold uppercase tracking-widest text-jira-muted">
                Projects ({projects.length})
              </h2>
              <div className="flex flex-wrap gap-2">
                {projects.map((p) => (
                  <Link
                    key={p.id}
                    href={`/projects/${p.id}`}
                    className="panel px-3 py-2 text-sm text-jira-text transition hover:border-jira-blue/50"
                  >
                    <span className="mr-2 font-bold text-jira-blue">{p.key}</span>
                    {p.name}
                  </Link>
                ))}
              </div>
            </section>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}