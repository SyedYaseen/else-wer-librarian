-- Add migration script here
-- Create authors table
CREATE TABLE authors ( id TEXT PRIMARY KEY, name TEXT );

-- Create works table
CREATE TABLE works (
    id TEXT PRIMARY KEY,
    title TEXT,
    author_id TEXT REFERENCES authors (id)
);

-- Create editions table
CREATE TABLE editions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    series TEXT,
    cover BIGINT
);

-- Create edition_authors table
CREATE TABLE edition_authors (
    edition_id TEXT REFERENCES editions (id),
    author_id TEXT REFERENCES authors (id),
    PRIMARY KEY (edition_id, author_id)
);