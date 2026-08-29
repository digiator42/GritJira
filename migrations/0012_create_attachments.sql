CREATE TABLE IF NOT EXISTS attachments (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    issue_id INTEGER NOT NULL,
    uploader_id INTEGER NOT NULL,
    filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(120) NOT NULL,
    size_bytes INTEGER NOT NULL DEFAULT 0,
    storage_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_attachments_issue FOREIGN KEY (issue_id)
        REFERENCES issues(id) ON DELETE CASCADE,
    CONSTRAINT fk_attachments_uploader FOREIGN KEY (uploader_id)
        REFERENCES users(id)
);

CREATE INDEX idx_attachments_issue_id ON attachments (issue_id);