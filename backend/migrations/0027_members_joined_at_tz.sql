-- project_members.joined_at is TIMESTAMP but the model reads DateTime<Utc>
-- (TIMESTAMPTZ). Naive wall-clock values are UTC; attach the zone explicitly.
-- One statement per file: the runner executes each file as a single prepared
-- statement.
ALTER TABLE project_members ALTER COLUMN joined_at TYPE TIMESTAMPTZ USING joined_at AT TIME ZONE 'UTC';
