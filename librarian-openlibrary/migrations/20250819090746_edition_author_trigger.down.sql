-- Add down migration script here
-- Drop the trigger and function
DROP TRIGGER IF EXISTS ensure_author_exists_for_edition_trigger ON edition_authors;

DROP FUNCTION IF EXISTS ensure_author_exists_for_edition ();