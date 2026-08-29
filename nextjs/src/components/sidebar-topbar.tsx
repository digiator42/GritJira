"use client";

import { useEffect, useRef, useState } from "react";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { useApp } from "@/lib/AppContext";
import { Dropdown } from "./Dropdown";
import { ThemeToggle } from "./ThemeToggle";
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
  { href: "/summary", label: "Summary" },
  { href: "/backlog", label: "Backlog" },
  { href: "/projects", label: "Projects" },
  { href: "/search", label: "Search" },
  { href: "/burndown", label: "Burndown" },
  { href: "/activity", label: "Activity" },
];

const SETTINGS = [
  { href: "/settings/general", label: "General" },
  { href: "/settings/issue-types", label: "Issue types" },
  { href: "/settings/users", label: "Users & members" },
  { href: "/settings/workflow", label: "Workflow" },
  { href: "/settings/webhooks", label: "Webhooks" },
];

function SidebarContent({
  pathname,
  onNavigate,
}: {
  pathname: string;
  onNavigate?: () => void;
}) {
  const isActive = (href: string) =>
    href === "/board"
      ? pathname === "/board"
      : pathname.startsWith(href) || (href === "/projects" && pathname.startsWith("/projects"));

  return (
    <>
      <div className="mb-5 flex items-center gap-2 px-2">
        <Link
          href="/board"
          onClick={onNavigate}
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
            onClick={onNavigate}
            className={`rounded-md px-2 py-1.5 text-sm transition ${
              isActive(item.href)
                ? "bg-jira-blue/20 font-medium text-white"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            {item.label}
          </Link>
        ))}
      </nav>
    </>
  );
}

export function Sidebar({
  pathname,
  mobileOpen,
  onClose,
}: {
  pathname: string;
  mobileOpen?: boolean;
  onClose?: () => void;
}) {
  return (
    <>
      <aside className="sticky top-0 hidden h-screen w-60 shrink-0 flex-col border-r border-jira-border bg-jira-panel px-3 py-4 md:flex">
        <SidebarContent pathname={pathname} />
      </aside>
      {mobileOpen ? (
        <div className="fixed inset-0 z-40 md:hidden">
          <div className="absolute inset-0 bg-black/60" onClick={onClose} aria-hidden />
          <aside className="absolute inset-y-0 left-0 mr-10 flex w-64 flex-col border-r border-jira-border bg-jira-panel px-3 py-4 shadow-2xl">
            <SidebarContent pathname={pathname} onNavigate={onClose} />
          </aside>
        </div>
      ) : null}
    </>
  );
}

export function Topbar({ onMenu }: { onMenu?: () => void }) {
  const { me, currentProject } = useApp();
  const router = useRouter();
  const pathname = usePathname();
  const [unread, setUnread] = useState(0);

  const doLogout = async () => {
    try {
      await fetch("/api/v1/auth/logout", { method: "POST", credentials: "include" });
    } finally {
      router.replace("/login");
      router.refresh();
    }
  };

  useEffect(() => {
    if (!currentProject) return;
    let cancelled = false;
    const poll = () => {
      fetch(`/api/v1/notifications/unread?project_id=${currentProject.id}`, {
        credentials: "include",
      })
        .then((r) => (r.ok ? r.json() : null))
        .then((body) => {
          if (!cancelled) setUnread(typeof body?.data === "number" ? body.data : 0);
        })
        .catch(() => {});
    };
    poll();
    const t = setInterval(poll, 20000);
    return () => {
      cancelled = true;
      clearInterval(t);
    };
  }, [currentProject]);

  const openPalette = () =>
    document.dispatchEvent(
      new KeyboardEvent("keydown", { key: "k", ctrlKey: true, bubbles: true }),
    );

  return (
    <header className="flex h-14 shrink-0 items-center gap-2 border-b border-jira-border bg-jira-panel px-3 sm:gap-3 sm:px-4">
      <button
        type="button"
        onClick={onMenu}
        aria-label="Open navigation menu"
        className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-jira-border text-jira-muted transition hover:border-jira-blue/50 hover:text-jira-text md:hidden"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
          <path d="M3 6h18M3 12h18M3 18h18" />
        </svg>
      </button>
      <div className="flex flex-1 items-center gap-2">
        <Link
          href="/board"
          className="hidden rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text md:inline-flex"
        >
          Dashboard
        </Link>
        <Link
          href="/projects"
          className="hidden rounded-md px-2 py-1 text-sm font-medium text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text md:inline-flex"
        >
          Projects
        </Link>
      </div>

      <button
        type="button"
        onClick={openPalette}
        className="flex items-center gap-2 rounded-md border border-jira-border bg-jira-bg px-3 py-1.5 text-sm text-jira-faint transition hover:border-jira-blue/50"
      >
        <span>⌕</span>
        <span className="hidden sm:inline">Search…</span>
        <kbd className="hidden rounded border border-jira-border bg-jira-panel px-1 text-[10px] text-jira-muted sm:block">
          Ctrl K
        </kbd>
      </button>

      <ThemeToggle />

      <Dropdown
        panelClassName="w-56"
        trigger={({ open, toggle }) => (
          <button
            type="button"
            title="Settings"
            aria-expanded={open}
            onClick={toggle}
            className={`flex h-8 w-8 items-center justify-center rounded-md border transition ${
              open
                ? "border-jira-blue/60 bg-jira-blue/15 text-jira-text"
                : "border-jira-border text-jira-muted hover:border-jira-blue/50 hover:text-jira-text"
            }`}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
          </button>
        )}
      >
        <p className="px-3 py-1.5 text-[10px] font-semibold uppercase tracking-widest text-jira-faint">
          Settings
        </p>
        {SETTINGS.map((item) => {
          const active = pathname.startsWith(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={`flex items-center gap-2 px-3 py-2 text-sm transition ${
                active
                  ? "bg-jira-blue/15 font-medium text-white"
                  : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
              }`}
            >
              {item.label}
            </Link>
          );
        })}
      </Dropdown>

      <Link
        href="/notifications"
        title="Notifications"
        className="relative flex h-8 w-8 items-center justify-center rounded-md border border-jira-border text-jira-muted transition hover:border-jira-blue/50 hover:text-jira-text"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M18 8a6 6 0 0 0-12 0c0 7-3 9-3 9h18s-3-2-3-9" />
          <path d="M13.73 21a2 2 0 0 1-3.46 0" />
        </svg>
        {unread > 0 ? (
          <span className="absolute -right-1 -top-1 flex h-4 min-w-4 items-center justify-center rounded-full bg-red-500 px-1 text-[9px] font-bold text-white">
            {unread > 9 ? "9+" : unread}
          </span>
        ) : null}
      </Link>

      <Dropdown
        panelClassName="w-60"
        trigger={({ open, toggle }) => (
          <button
            type="button"
            title="Account menu"
            aria-expanded={open}
            onClick={toggle}
            className="rounded-full transition hover:ring-2 hover:ring-jira-blue/60"
          >
            <span className="flex h-7 w-7 items-center justify-center rounded-full bg-gradient-to-br from-jira-blue to-purple-700 text-xs font-semibold text-white">
              {me ? initials(me.username) : "?"}
            </span>
          </button>
        )}
      >
        {me ? (
          <div className="border-b border-jira-border px-3 py-2">
            <p className="truncate text-sm font-medium text-jira-text">{me.username}</p>
            <p className="truncate text-xs text-jira-faint">
              {me.email} · {me.role}
            </p>
          </div>
        ) : null}
        <Link
          href="/settings/profile"
          className="flex items-center gap-2 px-3 py-2 text-sm text-jira-muted transition hover:bg-jira-border/40 hover:text-jira-text"
        >
          Profile
        </Link>
        <button
          onClick={() => void doLogout()}
          className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm text-red-400 transition hover:bg-red-500/10"
        >
          Log out
        </button>
      </Dropdown>
    </header>
  );
}