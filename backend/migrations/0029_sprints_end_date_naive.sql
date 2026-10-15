-- sprints.end_date is TIMESTAMPTZ but the model reads Option<NaiveDateTime>.
-- Same alignment direction as 0021 (schema follows the code).
ALTER TABLE sprints ALTER COLUMN end_date TYPE TIMESTAMP USING end_date AT TIME ZONE 'UTC';
