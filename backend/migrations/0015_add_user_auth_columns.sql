-- The 0001 users table predates auth: the model (and register/login/seeder)
-- requires role + password. Backfill-safe defaults for existing rows.
ALTER TABLE users ADD COLUMN IF NOT EXISTS role VARCHAR(50) NOT NULL DEFAULT 'Member';
ALTER TABLE users ADD COLUMN IF NOT EXISTS password VARCHAR(255) NOT NULL DEFAULT '';
