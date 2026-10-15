CREATE UNIQUE INDEX IF NOT EXISTS idx_issue_types_project_name ON issue_types (project_id, name);
