-- The Rust models read created_at as NaiveDateTime (TIMESTAMP), but the
-- table was created with TIMESTAMPTZ, which SeaORM cannot decode into it
-- (and the GritAdmin derive only formats NaiveDateTime). Align the column
-- with the code. One statement per file: the runner executes each file as a
-- single prepared statement. Existing values are UTC instants; the USING
-- clause preserves them wall-clock-identical.
ALTER TABLE users ALTER COLUMN created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
