// API types mirroring the Rust backend's serialized JSON.

export interface User {
  id: number;
  username: string;
  email: string;
  role: string;
  password?: string;
  avatar_url: string | null;
  created_at: string;
}

export interface Me {
  id: number;
  username: string;
  email: string;
  role: string;
  avatar_url: string | null;
  current_project_id: number | null;
  current_project_key: string | null;
}

export interface Project {
  id: number;
  key: string;
  name: string;
  description: string | null;
  created_at: string;
}

export interface Issue {
  id: number;
  project_id: number;
  sprint_id: number | null;
  step_id: number;
  reporter_id: number;
  assignee_id: number | null;
  key: string;
  summary: string;
  description: string | null;
  priority: number;
  issue_type: string;
  story_points: number | null;
  time_estimate_minutes: number | null;
  time_spent_minutes: number;
  created_at: string;
}

export interface Sprint {
  id: number;
  project_id: number;
  name: string;
  goal: string | null;
  status: string;
  start_date: string | null;
  end_date: string | null;
}

export interface WorkflowStep {
  id: number;
  project_id: number;
  name: string;
  position: number;
  is_completed: boolean;
}

export interface Comment {
  id: number;
  issue_id: number;
  author_id: number;
  body: string;
  created_at: string;
}

export interface ActivityLog {
  id: number;
  project_id: number;
  actor_id: number;
  action: string;
  issue_id: number | null;
  issue_key: string | null;
  summary: string | null;
  detail: string | null;
  target_user_id: number | null;
  is_read: boolean;
  created_at: string;
}

export interface NotificationsFeed {
  items: ActivityLog[];
  unread: number;
}

export interface BurndownPoint {
  date: string;
  remaining: number;
}

export interface BurndownColumn {
  id: number;
  name: string;
  is_completed: boolean;
  count: number;
  points: number;
}

export interface BurndownData {
  sprint: {
    id: number;
    name: string;
    status: string;
    start_date: string | null;
    end_date: string | null;
  };
  total_points: number;
  done_points: number;
  remaining_points: number;
  percent_done: number;
  columns: BurndownColumn[];
  ideal: BurndownPoint[];
  actual: BurndownPoint[];
}

export interface ProjectMember {
  id: number;
  project_id: number;
  user_id: number;
  username: string;
  role: string;
  joined_at: string;
}

export interface Webhook {
  id: number;
  project_id: number;
  name: string;
  url: string;
  event: string;
  is_active: boolean;
  created_at: string;
}

export const WEBHOOK_EVENTS = [
  { value: "issue.created", label: "Issue created" },
  { value: "issue.updated", label: "Issue updated" },
  { value: "issue.deleted", label: "Issue deleted" },
  { value: "issue.moved", label: "Issue moved" },
  { value: "issue.assigned", label: "Issue assigned" },
  { value: "*", label: "All events" },
];

export interface BoardColumn {
  step: WorkflowStep;
  issues: Issue[];
}

export interface BoardData {
  sprint_id: number;
  project_id: number;
  columns: BoardColumn[];
}

export interface BacklogData {
  backlog_issues: Issue[];
  sprints: Sprint[];
}

export interface Attachment {
  id: number;
  project_id: number;
  issue_id: number;
  uploader_id: number;
  filename: string;
  mime_type: string;
  size_bytes: number;
  storage_key: string;
  created_at: string;
}

export interface IssueDetail {
  issue: Issue;
  comments: Comment[];
  attachments: Attachment[];
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
}

export interface AuthResponse {
  success: boolean;
  message: string;
  user_id: number | null;
  role: string | null;
}

export type Priority =
  | 1
  | 2
  | 3
  | 4
  | 5;

export const PRIORITIES: { value: number; label: string }[] = [
  { value: 1, label: "Highest" },
  { value: 2, label: "High" },
  { value: 3, label: "Medium" },
  { value: 4, label: "Low" },
  { value: 5, label: "Lowest" },
];

export const ISSUE_TYPES = [
  "story",
  "bug",
  "task",
  "epic",
  "subtask",
];

export const MEMBER_ROLES = ["Admin", "Manager", "Developer", "Tester", "Viewer"];