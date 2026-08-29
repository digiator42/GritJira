
CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    project_id INT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    sprint_id INT REFERENCES sprints(id) ON DELETE SET NULL,
    step_id INT NOT NULL REFERENCES workflows(id),
    reporter_id INT NOT NULL REFERENCES users(id),
    assignee_id INT REFERENCES users(id) ON DELETE SET NULL,
    key VARCHAR(20) NOT NULL UNIQUE,
    summary VARCHAR(500) NOT NULL,
    description TEXT,
    priority VARCHAR(20) DEFAULT 'Medium',
    issue_type VARCHAR(20) DEFAULT 'Task',
    story_points INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

