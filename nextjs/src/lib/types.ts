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

export interface ProjectMember {
  id: number;
  project_id: number;
  user_id: number;
  username: string;
  role: string;
  joined_at: string;
}

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

export interface IssueDetail {
  issue: Issue;
  comments: Comment[];
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