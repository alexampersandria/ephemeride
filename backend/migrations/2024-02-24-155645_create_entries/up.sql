-- Your SQL goes here
CREATE TABLE
  entries (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users (id),
    date VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    mood INTEGER NOT NULL,
    entry VARCHAR(1023)
  )