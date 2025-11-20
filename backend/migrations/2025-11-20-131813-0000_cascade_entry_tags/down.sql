-- This file should undo anything in `up.sql`

-- Drop cascading foreign key constraints
ALTER TABLE entry_tags DROP CONSTRAINT IF EXISTS entry_tags_entry_id_fkey;
ALTER TABLE entry_tags DROP CONSTRAINT IF EXISTS entry_tags_tag_id_fkey;

-- Re-add foreign key constraints without CASCADE
ALTER TABLE entry_tags
ADD CONSTRAINT entry_tags_entry_id_fkey
FOREIGN KEY (entry_id) REFERENCES entries(id);

ALTER TABLE entry_tags
ADD CONSTRAINT entry_tags_tag_id_fkey
FOREIGN KEY (tag_id) REFERENCES tags(id);
