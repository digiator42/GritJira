
CREATE TABLE IF NOT EXISTS sprints (
    id SERIAL PRIMARY KEY,
    project_id INT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    goal TEXT,
    status VARCHAR(20) DEFAULT 'future',
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ
);
