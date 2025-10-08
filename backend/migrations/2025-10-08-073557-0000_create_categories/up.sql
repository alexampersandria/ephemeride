-- Your SQL goes here
CREATE TABLE
  categories (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL REFERENCES users (id),
    created_at BIGINT NOT NULL
  );