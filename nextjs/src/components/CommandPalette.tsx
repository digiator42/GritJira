"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import { useRouter } from "next/navigation";
import { apiData } from "@/lib/api";
import type { Issue } from "@/lib/types";
import { decodeEntities } from "@/lib/format";

interface Command {
  id: string;
  label: string;
  hint: string;
  glyph: string;
  go: () => void;
}

export function CommandPalette() {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [issues, setIssues] = useState<Issue[]>([]);
  const [active, setActive] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setOpen((v) => !v);
      }
      if (e.key === "Escape") setOpen(false);
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  useEffect(() => {
    if (open) {
      setQuery("");
      setIssues([]);
      setActive(0);
      setTimeout(() => inputRef.current?.focus(), 10);
    }
  }, [open]);

  useEffect(() => {
    if (!open) return;
    if (timer.current) clearTimeout(timer.current);
    const q = query.trim();
    if (!q) {
      setIssues([]);
      setActive(0);
      return;
    }
    timer.current = setTimeout(() => {
      apiData<Issue[]>(
        `/api/v1/issues/search?jql=${encodeURIComponent(`summary LIKE ${q}`)}`,
      )
        .then((found) => {
          setIssues(found.slice(0, 20));
          setActive(0);
        })
        .catch(() => setIssues([]));
    }, 200);
    return () => {
      if (timer.current) clearTimeout(timer.current);
    };
  }, [query, open]);

  const navCommands = useMemo<Command[]>(
    () => [
      { id: "board", label: "Board", hint: "page", glyph: "▦", go: () => router.push("/board") },
      { id: "backlog", label: "Backlog", hint: "page", glyph: "☰", go: () => router.push("/backlog") },
      { id: "search", label: "Search issues", hint: "page", glyph: "⌕", go: () => router.push("/search") },
      { id: "burndown", label: "Sprint burndown", hint: "page", glyph: "▁▃▅", go: () => router.push("/burndown") },
      { id: "activity", label: "Activity log", hint: "page", glyph: "◷", go: () => router.push("/activity") },
      { id: "notifications", label: "Notifications", hint: "page", glyph: "🔔", go: () => router.push("/notifications") },
      { id: "projects", label: "Projects", hint: "page", glyph: "▤", go: () => router.push("/projects") },
    ],
    [router],
  );

  const commands: Command[] = useMemo(() => {
    if (!query.trim())
      return navCommands;
    const heads = navCommands.filter((c) => c.label.toLowerCase().includes(query.toLowerCase()));
    return [...heads, ...issues.map((i): Command => ({
      id: `issue-${i.id}`,
      label: decodeEntities(i.summary),
      hint: i.key,
      glyph: "»",
      go: () => router.push(`/issues/${i.id}`),
    }))];
  }, [navCommands, issues, query]);

  useEffect(() => setActive(0), [commands.length, query]);

  if (!open) return null;

  const go = (cmd: Command) => {
    setOpen(false);
    cmd.go();
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setActive((v) => Math.min(v + 1, commands.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setActive((v) => Math.max(v - 1, 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      const cmd = commands[Math.min(active, commands.length - 1)];
      if (cmd) go(cmd);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center bg-black/50 p-4 pt-[12vh]"
      onMouseDown={(e) => {
        if (e.target === e.currentTarget) setOpen(false);
      }}
    >
      <div className="panel w-full max-w-lg overflow-hidden p-0">
        <div className="flex items-center gap-2 border-b border-jira-border px-3">
          <span className="text-sm text-jira-faint">⌕</span>
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Search issues or jump to a page…"
            className="w-full bg-transparent py-3 text-sm text-jira-text outline-none placeholder:text-jira-faint"
          />
          <kbd className="rounded border border-jira-border bg-jira-bg px-1.5 py-0.5 text-[10px] text-jira-faint">
            esc
          </kbd>
        </div>
        <ul className="max-h-80 overflow-y-auto py-1">
          {commands.length === 0 ? (
            <li className="px-3 py-3 text-xs text-jira-faint">
              No matches for “{query}”.
            </li>
          ) : (
            commands.map((cmd, i) => (
              <li key={cmd.id}>
                <button
                  type="button"
                  onMouseEnter={() => setActive(i)}
                  onClick={() => go(cmd)}
                  className={`flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition ${
                    i === active ? "bg-jira-blue/20" : ""
                  }`}
                >
                  <span className="flex h-5 w-5 shrink-0 items-center justify-center text-xs text-jira-muted">
                    {cmd.glyph}
                  </span>
                  <span className="min-w-0 flex-1 truncate text-jira-text">
                    {decodeEntities(cmd.label)}
                  </span>
                  <span className="shrink-0 text-[10px] font-medium text-jira-faint">{cmd.hint}</span>
                </button>
              </li>
            ))
          )}
        </ul>
        <div className="flex items-center gap-3 border-t border-jira-border px-3 py-1.5 text-[10px] text-jira-faint">
          <span>
            <kbd className="rounded border border-jira-border px-1">↑</kbd>/
            <kbd className="rounded border border-jira-border px-1">↓</kbd> navigate
          </span>
          <span>
            <kbd className="rounded border border-jira-border px-1">↵</kbd> open
          </span>
          <span className="ml-auto">Search: summary LIKE</span>
        </div>
      </div>
    </div>
  );
}