-- This file should undo anything in `up.sql`
ALTER TABLE entries
ADD COLUMN date_old VARCHAR(255);

UPDATE entries
SET
  date_old = CAST(date AS VARCHAR);

ALTER TABLE entries
DROP COLUMN date;

ALTER TABLE entries
RENAME COLUMN date_old TO date;

ALTER TABLE entries
ALTER COLUMN date
SET
  NOT NULL;