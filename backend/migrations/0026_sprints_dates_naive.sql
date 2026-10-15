-- Same NaiveDateTime/TIMESTAMPTZ alignment as 0021 (see note there).
-- One ALTER TABLE statement carrying both column actions (still a single
-- prepared statement for the runner).
ALTER TABLE sprints ALTER COLUMN start_date TYPE TIMESTAMP USING start_date AT TIME ZONE 'UTC', ALTER COLUMN end_date TYPE TIMESTAMP USING end_date AT TIME ZONE 'UTC';
