-- 1. Creates the `entities` table. An entity is an atomic data point.
CREATE TABLE entities (
    -- Columns
    id BLOB PRIMARY KEY, -- UUID v7
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Creates the `components` table. A component is a data section of
-- an entity.
CREATE TABLE components (
    -- Columns
    id BLOB PRIMARY KEY, -- UUID v7
    component_name TEXT NOT NULL,
    dev_color INTEGER NOT NULL, -- only for the admin panel
    is_system INTEGER NOT NULL, -- boolean
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Creates the `fields` table. A field is a data row in a component.
CREATE TABLE fields (
    -- Columns
    id BLOB PRIMARY KEY, -- UUID v7
    component_id BLOB NOT NULL,
    field_name TEXT NOT NULL,
    field_type TEXT NOT NULL,
    is_system INTEGER NOT NULL, -- boolean
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Other
    FOREIGN KEY (component_id) REFERENCES components(id)
);

-- 4. Creates the `logs` table, for general recording purposes.
-- TODO
