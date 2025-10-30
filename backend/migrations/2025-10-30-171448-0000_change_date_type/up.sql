-- Your SQL goes here
ALTER TABLE entries
ADD COLUMN date_new DATE;

UPDATE entries
SET
  date_new = CAST(date AS DATE);

ALTER TABLE entries
DROP COLUMN date;

ALTER TABLE entries
RENAME COLUMN date_new TO date;

ALTER TABLE entries
ALTER COLUMN date
SET
  NOT NULL;