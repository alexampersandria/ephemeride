-- Your SQL goes here
-- when entries are deleted or tags are deleted, the corresponding
-- entry_tags are automatically deleted via cascading foreign key constraints

-- Drop existing foreign key constraints
ALTER TABLE entry_tags DROP CONSTRAINT IF EXISTS entry_tags_entry_id_fkey;
ALTER TABLE entry_tags DROP CONSTRAINT IF EXISTS entry_tags_tag_id_fkey;

-- Re-add foreign key constraints with ON DELETE CASCADE
ALTER TABLE entry_tags
ADD CONSTRAINT entry_tags_entry_id_fkey
FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE;

ALTER TABLE entry_tags
ADD CONSTRAINT entry_tags_tag_id_fkey
FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE;
