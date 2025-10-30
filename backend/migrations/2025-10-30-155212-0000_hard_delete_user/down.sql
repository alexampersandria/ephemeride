-- This file should undo anything in `up.sql`
ALTER TABLE users
ADD COLUMN deleted BOOLEAN NOT NULL DEFAULT FALSE;