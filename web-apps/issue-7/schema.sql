-- Create the articles table if it doesn't exist
CREATE TABLE IF NOT EXISTS articles (
    id INTEGER PRIMARY KEY,
    title VARCHAR(255),
    content TEXT,
    published_date VARCHAR(255)
);
