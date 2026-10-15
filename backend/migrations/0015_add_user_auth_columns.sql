-- The 0001 users table predates auth: the model (and register/login/seeder)
-- requires role + password. One statement per file: the runner executes each
-- file as a single prepared statement. Backfill-safe default for existing rows.
ALTER TABLE users ADD COLUMN IF NOT EXISTS role VARCHAR(50) NOT NULL DEFAULT 'Member';
