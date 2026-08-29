CREATE TABLE IF NOT EXISTS activity_logs (
    id SERIAL PRIMARY KEY,
    project_id INT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    actor_id INT NOT NULL REFERENCES users(id),
    action VARCHAR(64) NOT NULL,
    issue_id INT REFERENCES issues(id) ON DELETE SET NULL,
    issue_key VARCHAR(32),
    summary TEXT,
    detail TEXT,
    target_user_id INT,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);