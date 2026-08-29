"use client";

import type { IssueType, User } from "@/lib/types";
import { DEFAULT_ISSUE_TYPE_STYLES } from "@/lib/types";
import { Avatar } from "./ui";
import { Dropdown } from "./Dropdown";
import { IssueTypeIcon, resolveIssueType } from "./IssueTypeIcon";

export function TypePicker({
  value,
  onChange,
  options,
}: {
  value: string;
  onChange: (v: string) => void;
  options: IssueType[];
}) {
  const names =
    options.length > 0 ? options.map((t) => t.name) : Object.keys(DEFAULT_ISSUE_TYPE_STYLES);
  const selected = resolveIssueType(value, options);
  return (
    <Dropdown
      align="left"
      panelClassName="w-full"
      trigger={({ open, toggle }) => (
        <button
          type="button"
          onClick={toggle}
          className={`input flex items-center justify-between gap-2 !text-left ${
            open ? "!border-jira-blue" : ""
          }`}
        >
          <span className="inline-flex min-w-0 items-center gap-2">
            <IssueTypeIcon iconKey={selected.icon_key} color={selected.color} size={16} />
            <span className="truncate text-sm capitalize text-jira-text">{selected.label}</span>
          </span>
          <Chevron open={open} />
        </button>
      )}
    >
      {(close) => (
        <div className="max-h-60 overflow-auto py-1">
          {names.map((name) => {
            const s = resolveIssueType(name, options);
            const active = value.toLowerCase() === name.toLowerCase();
            return (
              <button
                key={name}
                type="button"
                onClick={() => {
                  onChange(name);
                  close();
                }}
                className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
                  active
                    ? "bg-jira-blue/15 font-medium text-jira-text"
                    : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
                }`}
              >
                <IssueTypeIcon iconKey={s.icon_key} color={s.color} size={16} />
                <span className="flex-1 truncate text-left capitalize">{s.label}</span>
                {active ? (
                  <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
                    <path
                      d="m5 13 4 4L19 7"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                ) : null}
              </button>
            );
          })}
        </div>
      )}
    </Dropdown>
  );
}

export function AssigneePicker({
  users,
  value,
  onChange,
}: {
  users: User[];
  value: string;
  onChange: (v: string) => void;
}) {
  const selected = users.find((u) => String(u.id) === value);
  return (
    <Dropdown
      align="left"
      panelClassName="w-full"
      trigger={({ open, toggle }) => (
        <button
          type="button"
          onClick={toggle}
          className={`input flex items-center justify-between gap-2 !text-left ${
            open ? "!border-jira-blue" : ""
          }`}
        >
          <span className="inline-flex min-w-0 items-center gap-2">
            {selected ? (
              <Avatar name={selected.username} size={20} />
            ) : (
              <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-jira-border/60 text-[10px] text-jira-faint">
                ?
              </span>
            )}
            <span className="truncate text-sm text-jira-text">
              {selected ? selected.username : "Unassigned"}
            </span>
          </span>
          <Chevron open={open} />
        </button>
      )}
    >
      {(close) => (
        <div className="max-h-56 overflow-auto py-1">
          <button
            type="button"
            onClick={() => {
              onChange("");
              close();
            }}
            className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
              value === ""
                ? "bg-jira-blue/15 font-medium text-jira-text"
                : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
            }`}
          >
            <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-jira-border/60 text-[10px] text-jira-faint">
              ?
            </span>
            <span className="flex-1 truncate text-left text-jira-muted">Unassigned</span>
            {value === "" ? (
              <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
                <path
                  d="m5 13 4 4L19 7"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            ) : null}
          </button>
          {users.map((u) => {
            const active = String(u.id) === value;
            return (
              <button
                key={u.id}
                type="button"
                onClick={() => {
                  onChange(String(u.id));
                  close();
                }}
                className={`flex w-full items-center gap-2 px-3 py-2 text-sm ${
                  active
                    ? "bg-jira-blue/15 font-medium text-jira-text"
                    : "text-jira-muted hover:bg-jira-border/40 hover:text-jira-text"
                }`}
              >
                <Avatar name={u.username} size={20} />
                <span className="flex-1 truncate text-left">{u.username}</span>
                <span className="shrink-0 text-[10px] uppercase tracking-wide text-jira-faint">
                  {u.role}
                </span>
                {active ? (
                  <svg viewBox="0 0 24 24" className="h-4 w-4 shrink-0 text-jira-blue" fill="none">
                    <path
                      d="m5 13 4 4L19 7"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    />
                  </svg>
                ) : null}
              </button>
            );
          })}
        </div>
      )}
    </Dropdown>
  );
}

function Chevron({ open }: { open: boolean }) {
  return (
    <svg
      viewBox="0 0 24 24"
      className={`h-4 w-4 shrink-0 text-jira-faint transition-transform ${open ? "rotate-180" : ""}`}
      fill="none"
    >
      <path d="m6 9 6 6 6-6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}