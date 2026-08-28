"use client";

import { useRouter } from "next/navigation";
import { useApp } from "@/lib/AppContext";
import { Avatar } from "./ui";
import { initials } from "@/lib/format";

export function ProjectSwitcher() {
  const { projects, currentProject, selectProject } = useApp();

  return (
    <div className="mb-4">
      <label className="label">Project</label>
      <select
        className="input"
        value={currentProject?.id ?? ""}
        onChange={(e) => {
          const id = Number(e.target.value);
          if (Number.isFinite(id)) selectProject(id);
        }}
      >
        {projects.map((p) => (
          <option key={p.id} value={p.id}>
            {p.key} — {p.name}
          </option>
        ))}
      </select>
    </div>
  );
}

const NAV = [
  { href: "/board", label: "Board" },
  { href: "/backlog", label: "Backlog" },
  { href: "/projects", label: "Projects" },
  { href: "/search", label: "Search" },
];

const SETTINGS = [
  { href: "/settings/users", label: "Users & Members" },
  { href: "/settings/workflow", label: "Workflow" },
];

export function Sidebar({ pathname }: { pathname: string }) {
  const { me } = useApp();
  const isActive = (href: string) =>
    href === "/board"
      ? pathname === "/board"
      : pathname.startsWith(href) || (href === "/projects" && pathname.startsWith("/projects"));

  return (
    <aside className="sticky top-0 flex h-screen w-60 shrink-0 flex-col border-r border-jira-border bg-jira-panel px-3 py-4">
      <div className="mb-5 flex items-center gap-2 px-2">
        <span className="flex h-7 w-7 items-center justify-center rounded bg-jira-blue text-sm font-bold text-white">
          G
        </span>
        <span className="text-sm font-bold tracking-tight text-jira-text">Grit Jira</span>
      </div>

      <ProjectSwitcher />

      <nav className="flex flex-1 flex-col gap-0.5">
        {NAV.map((item) => (
          <a
            key={item.href}
            href={item.href}
            className={`rounded-md px-2 py-1.5 text-sm transition ${
              isActive(item.href)
                ? "bg-jira-blue/20 font-medium text-white"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            {item.label}
          </a>
        ))}

        <p className="mb-1 mt-5 px-2 text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          Project settings
        </p>
        {SETTINGS.map((item) => (
          <a
            key={item.href}
            href={item.href}
            className={`rounded-md px-2 py-1.5 text-sm transition ${
              pathname.startsWith(item.href)
                ? "bg-jira-blue/20 font-medium text-white"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            {item.label}
          </a>
        ))}
      </nav>

      {me ? (
        <div className="mt-4 flex items-center gap-2 border-t border-jira-border px-2 pt-3">
          <Avatar name={me.username} size={28} />
          <div className="min-w-0 flex-1">
            <p className="truncate text-xs font-medium text-jira-text">{me.username}</p>
            <p className="truncate text-[10px] text-jira-faint">{me.role}</p>
          </div>
        </div>
      ) : null}
    </aside>
  );
}

export function Topbar() {
  const { me } = useApp();
  const router = useRouter();

  return (
    <header className="flex h-14 shrink-0 items-center gap-3 border-b border-jira-border bg-jira-panel px-4">
      <div className="flex flex-1 items-center gap-2">
        <a
          href="/board"
          className="rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text"
        >
          Dashboard
        </a>
        <a
          href="/projects"
          className="rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text"
        >
          Projects
        </a>
      </div>

      <a
        href="/search"
        className="flex items-center gap-2 rounded-md border border-jira-border bg-jira-bg px-3 py-1.5 text-sm text-jira-faint transition hover:border-jira-blue/50"
      >
        <span>⌕</span> Search issues…
      </a>

      <div className="flex items-center gap-3">
        <span className="flex h-7 w-7 items-center justify-center rounded-md bg-gradient-to-br from-jira-blue to-purple-700 text-xs font-semibold text-white">
          {me ? initials(me.username) : "?"}
        </span>
        <button
          onClick={async () => {
            try {
              await fetch("/api/v1/auth/logout", { method: "POST", credentials: "include" });
            } finally {
              router.replace("/login");
              router.refresh();
            }
          }}
          className="rounded-md px-2 py-1 text-sm text-jira-muted transition hover:text-jira-text"
        >
          Log out
        </button>
      </div>
    </header>
  );
}