"use client";

import { useEffect } from "react";
import { initials, normalizeSprintStatus, priorityLabel } from "@/lib/format";
import { IssueTypeBadge } from "./IssueTypeIcon";

export function Spinner({ label }: { label?: string }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-16 text-jira-muted">
      <div className="h-8 w-8 animate-spin rounded-full border-2 border-jira-border border-t-jira-blue" />
      {label ? <p className="text-sm">{label}</p> : null}
    </div>
  );
}

export function ErrorBox({ message }: { message: string }) {
  return (
    <div className="panel border-red-900/50 bg-red-950/20 p-4 text-sm text-red-300">
      {message}
    </div>
  );
}

export function EmptyState({ title, hint }: { title: string; hint?: string }) {
  return (
    <div className="flex flex-col items-center justify-center gap-2 py-14 text-center">
      <p className="text-sm font-medium text-jira-muted">{title}</p>
      {hint ? <p className="text-xs text-jira-faint">{hint}</p> : null}
    </div>
  );
}

export function Avatar({
  name,
  size = 28,
  className = "",
}: {
  name: string;
  size?: number;
  className?: string;
}) {
  return (
    <span
      className={`inline-flex shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-jira-blue to-purple-700 font-semibold text-white ${className}`}
      style={{ width: size, height: size, fontSize: Math.round(size * 0.38) }}
      title={name}
    >
      {initials(name || "?")}
    </span>
  );
}

export function PriorityBadge({ value }: { value: number }) {
  const styles: Record<number, string> = {
    1: "bg-red-500/15 text-red-400 border-red-500/30",
    2: "bg-orange-500/15 text-orange-400 border-orange-500/30",
    3: "bg-yellow-500/15 text-yellow-300 border-yellow-500/30",
    4: "bg-green-500/15 text-green-400 border-green-500/30",
    5: "bg-gray-500/15 text-gray-400 border-gray-500/30",
  };
  return (
    <span
      className={`rounded border px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${
        styles[value] ?? styles[5]
      }`}
    >
      {priorityLabel(value)}
    </span>
  );
}

export function TypeBadge({ type }: { type: string }) {
  return <IssueTypeBadge type={type} />;
}

export function SprintStatusBadge({ status }: { status: string }) {
  const s = normalizeSprintStatus(status);
  const styles: Record<string, string> = {
    Active: "bg-emerald-500/15 text-emerald-300 border-emerald-500/30",
    Completed: "bg-gray-500/15 text-gray-400 border-gray-500/30",
    Planning: "bg-blue-500/15 text-blue-300 border-blue-500/30",
  };
  return (
    <span
      className={`rounded border px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${
        styles[s] ?? "bg-gray-500/15 text-gray-400 border-gray-500/30"
      }`}
    >
      {s}
    </span>
  );
}

export function Modal({
  open,
  onClose,
  title,
  children,
  wide = false,
}: {
  open: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  wide?: boolean;
}) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  if (!open) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
      onMouseDown={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
    >
      <div
        className={`panel w-full ${wide ? "max-w-2xl" : "max-w-md"} max-h-[88vh] overflow-y-auto`}
      >
        <div className="flex items-center justify-between border-b border-jira-border px-4 py-3">
          <h3 className="text-sm font-semibold text-jira-text">{title}</h3>
          <button
            onClick={onClose}
            className="rounded p-1 text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            aria-label="Close"
          >
            ✕
          </button>
        </div>
        <div className="p-4">{children}</div>
      </div>
    </div>
  );
}

export function Field({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <label className="label">{label}</label>
      {children}
    </div>
  );
}