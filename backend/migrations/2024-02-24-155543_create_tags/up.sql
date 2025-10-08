-- Your SQL goes here
CREATE TABLE
  tags (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users (id),
    created_at BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    icon VARCHAR(255) NOT NULL
  )