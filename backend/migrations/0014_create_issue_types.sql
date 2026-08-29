CREATE TABLE IF NOT EXISTS issue_types (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    icon_key VARCHAR(50) NOT NULL DEFAULT 'task',
    color VARCHAR(20) NOT NULL DEFAULT '#4bade9',
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_issue_types_project FOREIGN KEY (project_id)
        REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX idx_issue_types_project_id ON issue_types (project_id);
CREATE UNIQUE INDEX idx_issue_types_project_name ON issue_types (project_id, name);