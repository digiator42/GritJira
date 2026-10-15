-- Same NaiveDateTime/TIMESTAMPTZ alignment as 0021 (see note there).
ALTER TABLE projects ALTER COLUMN created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
