"use client";

import type { Issue } from "@/lib/types";
import { PriorityBadge, TypeBadge, Avatar } from "./ui";
import { userById } from "@/lib/format";

export function IssueCard({
  issue,
  users,
  onClick,
  draggable = false,
  onDragStart,
  onDragOver,
  onDrop,
}: {
  issue: Issue;
  users: { id: number; username: string }[];
  onClick?: () => void;
  draggable?: boolean;
  onDragStart?: (e: React.DragEvent) => void;
  onDragOver?: (e: React.DragEvent) => void;
  onDrop?: (e: React.DragEvent) => void;
}) {
  return (
    <div
      draggable={draggable}
      onDragStart={onDragStart}
      onDragOver={onDragOver}
      onDrop={onDrop}
      onClick={onClick}
      className="group cursor-pointer rounded-md border border-jira-border bg-[#1c2129] p-2.5 transition hover:border-jira-blue/60 hover:bg-[#20262f]"
      title={issue.summary}
    >
      <div className="mb-1.5 flex items-start justify-between gap-2">
        <span className="text-xs font-medium text-jira-faint">{issue.key}</span>
        <PriorityBadge value={issue.priority} />
      </div>
      <p className="mb-2 line-clamp-2 text-sm leading-snug text-jira-text">
        {issue.summary}
      </p>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1.5">
          <TypeBadge type={issue.issue_type} />
          {issue.story_points != null && (
            <span className="text-[10px] text-jira-faint">${issue.story_points}</span>
          )}
        </div>
        <Avatar name={userById(users, issue.assignee_id)} size={22} />
      </div>
    </div>
  );
}