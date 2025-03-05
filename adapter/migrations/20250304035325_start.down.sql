-- Add down migration script here
-- make sure write statements invert order
DROP TRIGGER IF EXISTS books_updated_at_trigger;
DROP IF EXISTS books;

DROP FUNCTION set_updated_at;
