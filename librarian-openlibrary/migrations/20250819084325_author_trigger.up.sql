-- Add up migration script here
CREATE OR REPLACE FUNCTION ensure_author_exists()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if author exists
    IF NOT EXISTS (SELECT 1 FROM authors WHERE id = NEW.author_id) THEN
        INSERT INTO authors (id, name)
        VALUES (NEW.author_id, 'Unknown')
        ON CONFLICT (id) DO NOTHING;  -- in case of race condition
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ensure_author_exists_trigger
BEFORE INSERT OR UPDATE ON works
FOR EACH ROW
EXECUTE FUNCTION ensure_author_exists();