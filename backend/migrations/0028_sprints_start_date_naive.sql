-- sprints.start_date is TIMESTAMPTZ but the model reads Option<NaiveDateTime>.
-- Same alignment direction as 0021 (schema follows the code). NULLs pass
-- through the USING clause untouched.
ALTER TABLE sprints ALTER COLUMN start_date TYPE TIMESTAMP USING start_date AT TIME ZONE 'UTC';
