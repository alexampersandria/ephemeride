-- Your SQL goes here
CREATE TABLE
  invites (
    id VARCHAR(255) PRIMARY KEY,
    created_at BIGINT NOT NULL,
    code VARCHAR(255) NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE
  )