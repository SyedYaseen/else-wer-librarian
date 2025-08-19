-- Add down migration script here
DROP TRIGGER IF EXISTS ensure_author_exists_trigger ON works;

-- Then drop the function that the trigger calls
DROP FUNCTION IF EXISTS ensure_author_exists ();