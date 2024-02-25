-- Your SQL goes here
CREATE TABLE entry_tags (
  id VARCHAR(255) PRIMARY KEY,
  entry_id VARCHAR(255) NOT NULL REFERENCES entries(id),
  tag_id VARCHAR(255) NOT NULL REFERENCES tags(id)
)