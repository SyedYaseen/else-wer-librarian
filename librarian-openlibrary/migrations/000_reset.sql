-- Drop triggers
-- Reset migration tracking (optional)
DELETE FROM _sqlx_migrations;

DROP TRIGGER IF EXISTS ensure_author_exists_trigger ON works;

DROP FUNCTION IF EXISTS ensure_author_exists ();

DROP TRIGGER IF EXISTS ensure_author_exists_for_edition_trigger ON edition_authors;

DROP FUNCTION IF EXISTS ensure_author_exists_for_edition ();

-- Drop tables
DROP TABLE IF EXISTS edition_authors;

DROP TABLE IF EXISTS editions;

DROP TABLE IF EXISTS works;

DROP TABLE IF EXISTS authors;