-- This file should undo anything in `up.sql`
ALTER TABLE tags
DROP COLUMN color;

ALTER TABLE tags
DROP COLUMN category_id;

ALTER TABLE tags
ADD COLUMN icon VARCHAR(255) NOT NULL DEFAULT 'tag';