-- Companion to 0015 (one statement per file; see note there).
-- Empty hash never verifies, so backfilled rows stay inert until set.
ALTER TABLE users ADD COLUMN IF NOT EXISTS password VARCHAR(255) NOT NULL DEFAULT '';
