-- Postgres
-- CREATE TABLE IF NOT EXISTS articles (
--     id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
--     title VARCHAR(255),
--     content TEXT,
--     published_date VARCHAR(255)
-- );

-- SQLite
-- NOTE: This is just a workaround for UUID. The randomblob function generates
-- random binary data, and we convert it to a hexadecimal string to mimic a
-- UUID.
CREATE TABLE IF NOT EXISTS articles (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-6' || substr(lower(hex(randomblob(6))),2)),
    title TEXT,
    content TEXT,
    published_date TEXT
);
