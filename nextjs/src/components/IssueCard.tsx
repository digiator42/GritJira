"use client";

import type { Issue } from "@/lib/types";
import { PriorityBadge, Avatar } from "./ui";
import { userById, decodeEntities } from "@/lib/format";
import { IssueTypeBadge } from "./IssueTypeIcon";
import { useApp } from "@/lib/AppContext";

export function IssueCard({
  issue,
  users,
  onClick,
  draggable = false,
  onDragStart,
  onDragOver,
  onDrop,
  onDragEnd,
}: {
  issue: Issue;
  users: { id: number; username: string }[];
  onClick?: () => void;
  draggable?: boolean;
  onDragStart?: (e: React.DragEvent) => void;
  onDragOver?: (e: React.DragEvent) => void;
  onDrop?: (e: React.DragEvent) => void;
  onDragEnd?: (e: React.DragEvent) => void;
}) {
  const { issueTypes } = useApp();
  return (
    <div
      draggable={draggable}
      onDragStart={onDragStart}
      onDragOver={onDragOver}
      onDrop={onDrop}
      onDragEnd={onDragEnd}
      onClick={onClick}
      className="group cursor-pointer rounded-md border border-jira-border bg-jira-card p-2.5 transition hover:border-jira-blue/60 hover:bg-jira-card-hover"
      title={decodeEntities(issue.summary)}
    >
      <div className="mb-1.5 flex items-start justify-between gap-2">
        <span className="text-xs font-medium text-jira-faint">{issue.key}</span>
        <PriorityBadge value={issue.priority} />
      </div>
      <p className="mb-2 line-clamp-2 text-sm leading-snug text-jira-text">
        {decodeEntities(issue.summary)}
      </p>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1.5">
          <IssueTypeBadge type={issue.issue_type} issueTypes={issueTypes} size={15} />
          {issue.story_points != null && (
            <span className="text-[10px] text-jira-faint">${issue.story_points}</span>
          )}
          {issue.due_date && (
            <span
              className={`text-[10px] ${issue.due_date < new Date().toISOString().slice(0, 10) ? "text-red-400" : "text-jira-faint"}`}
            >
              ⏱ {issue.due_date}
            </span>
          )}
        </div>
        <Avatar name={userById(users, issue.assignee_id)} size={22} />
      </div>
    </div>
  );
}