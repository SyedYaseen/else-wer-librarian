-- Up migration
CREATE OR REPLACE FUNCTION ensure_author_exists_for_edition()
RETURNS TRIGGER AS $$
BEGIN
    -- If author doesn’t exist, insert the missing author_id with 'Unknown' name
    IF NOT EXISTS (SELECT 1 FROM authors WHERE id = NEW.author_id) THEN
        INSERT INTO authors (id, name)
        VALUES (NEW.author_id, 'Unknown')
        ON CONFLICT (id) DO NOTHING;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger on edition_authors
CREATE TRIGGER ensure_author_exists_for_edition_trigger
BEFORE INSERT ON edition_authors
FOR EACH ROW
EXECUTE FUNCTION ensure_author_exists_for_edition();