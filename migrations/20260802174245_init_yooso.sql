-- 1. Creates the `entities` table with UUID v7 primary key and storing
-- when the entity was created.
CREATE TABLE entities (
    id BLOB PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- TODO `components`
