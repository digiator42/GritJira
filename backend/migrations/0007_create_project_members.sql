-- Create project_members table
CREATE TABLE IF NOT EXISTS project_members (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username VARCHAR(50) NOT NULL DEFAULT 'Role',
    role VARCHAR(50) NOT NULL DEFAULT 'Member',
    joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, user_id)
);

-- Add indexes
-- CREATE INDEX idx_project_members_project_id ON project_members(project_id);
-- CREATE INDEX idx_project_members_user_id ON project_members(user_id);