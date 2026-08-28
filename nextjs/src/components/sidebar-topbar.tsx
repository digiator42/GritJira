"use client";

import { useEffect, useRef, useState } from "react";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { useApp } from "@/lib/AppContext";
import { Avatar } from "./ui";
import { initials } from "@/lib/format";

const PALETTE = ["#3b82f6", "#a855f7", "#10b981", "#f59e0b", "#ef4444", "#06b6d4", "#ec4899"];

function hashString(s: string): number {
  let h = 0;
  for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) | 0;
  return Math.abs(h);
}

export function ProjectAvatar({ project, size = 22 }: { project: { key: string }; size?: number }) {
  const color = PALETTE[hashString(project.key) % PALETTE.length];
  const width = `${size}px`;
  return (
    <span
      className="flex shrink-0 items-center justify-center rounded font-semibold text-white"
      style={{ width, height: width, fontSize: Math.max(10, Math.round(size * 0.5)), backgroundColor: color }}
    >
      {project.key.slice(0, 1)}
    </span>
  );
}

export function ProjectSwitcher() {
  const { projects, currentProject, selectProject } = useApp();
  const router = useRouter();
  const pathname = usePathname();
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointer = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") setOpen(false);
    };
    document.addEventListener("mousedown", onPointer);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onPointer);
      document.removeEventListener("keydown", onKey);
    };
  }, [open]);

  const pick = (id: number) => {
    selectProject(id);
    setOpen(false);
    const base = pathname === "/" ? "/board" : pathname;
    router.push(`${base}?project_id=${id}`);
  };

  return (
    <div ref={ref} className="relative mb-4">
      <label className="label">Project</label>
      <button
        type="button"
        aria-haspopup="listbox"
        aria-expanded={open}
        onClick={() => setOpen((v) => !v)}
        className="flex w-full items-center gap-2 rounded-md border border-jira-border bg-jira-bg px-2 py-1.5 text-left transition hover:border-jira-blue/50"
      >
        {currentProject ? (
          <>
            <ProjectAvatar project={currentProject} size={22} />
            <span className="min-w-0 flex-1 truncate text-sm text-jira-text">
              {currentProject.key} — {currentProject.name}
            </span>
          </>
        ) : (
          <span className="text-sm text-jira-faint">Select project…</span>
        )}
        <svg
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2.5"
          className={`shrink-0 text-jira-muted transition ${open ? "rotate-180" : ""}`}
        >
          <path d="m6 9 6 6 6-6" strokeLinecap="round" strokeLinejoin="round" />
        </svg>
      </button>

      {open && (
        <div
          role="listbox"
          className="absolute left-0 right-0 z-50 mt-1 max-h-80 overflow-auto rounded-md border border-jira-border bg-jira-panel py-1 shadow-2xl"
        >
          {projects.length === 0 ? (
            <p className="px-3 py-3 text-xs text-jira-faint">No projects available.</p>
          ) : (
            projects.map((p) => {
              const active = p.id === currentProject?.id;
              return (
                <button
                  key={p.id}
                  type="button"
                  role="option"
                  aria-selected={active}
                  onClick={() => pick(p.id)}
                  className={`flex w-full items-center gap-2 px-2.5 py-1.5 text-left text-sm transition hover:bg-jira-border/40 ${
                    active ? "bg-jira-blue/15" : ""
                  }`}
                >
                  <ProjectAvatar project={p} size={20} />
                  <span className="min-w-0 flex-1 truncate text-jira-text">
                    <span className="font-medium">{p.key}</span>
                    <span className="text-jira-faint"> — {p.name}</span>
                  </span>
                  {active && <span className="shrink-0 text-xs text-jira-blue">✓</span>}
                </button>
              );
            })
          )}
        </div>
      )}
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
        <Link
          href="/board"
          className="flex h-7 w-7 items-center justify-center rounded bg-jira-blue text-sm font-bold text-white"
        >
          G
        </Link>
        <span className="text-sm font-bold tracking-tight text-jira-text">Grit Jira</span>
      </div>

      <ProjectSwitcher />

      <nav className="flex flex-1 flex-col gap-0.5">
        {NAV.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={`rounded-md px-2 py-1.5 text-sm transition ${
              isActive(item.href)
                ? "bg-jira-blue/20 font-medium text-white"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            {item.label}
          </Link>
        ))}

        <p className="mb-1 mt-5 px-2 text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          Project settings
        </p>
        {SETTINGS.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={`rounded-md px-2 py-1.5 text-sm transition ${
              pathname.startsWith(item.href)
                ? "bg-jira-blue/20 font-medium text-white"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            {item.label}
          </Link>
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
        <Link
          href="/board"
          className="rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text"
        >
          Dashboard
        </Link>
        <Link
          href="/projects"
          className="rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text"
        >
          Projects
        </Link>
      </div>

      <Link
        href="/search"
        className="flex items-center gap-2 rounded-md border border-jira-border bg-jira-bg px-3 py-1.5 text-sm text-jira-faint transition hover:border-jira-blue/50"
      >
        <span>⌕</span> Search issues…
      </Link>

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