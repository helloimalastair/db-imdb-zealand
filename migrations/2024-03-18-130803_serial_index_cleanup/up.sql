-- Set ID Values
SELECT SETVAL('titles_tconst_seq', (SELECT MAX(tconst) + 1 FROM titles));
SELECT SETVAL('names_nconst_seq', (SELECT MAX(nconst) + 1+ 1  + 1 FROM names));
-- Create Indexes
CREATE INDEX names_primary_index ON names (primaryname);
CREATE INDEX titles_primary ON titles (primarytitle);
CREATE INDEX titles_original ON titles (originaltitle);
-- Drop Orphans
DELETE FROM "aka_attributes" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst);
DELETE FROM "aka_types" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst);
DELETE FROM "akas" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst);
DELETE FROM "crew" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst) OR NOT EXISTS (SELECT 1 FROM "names" WHERE nconst = a.nconst);
DELETE FROM "genres" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst);
DELETE FROM "principals" AS a WHERE NOT EXISTS (SELECT 1 FROM "titles" WHERE tconst = a.tconst) OR NOT EXISTS (SELECT 1 FROM "names" WHERE nconst = a.nconst);
VACUUM FULL aka_attributes;
VACUUM FULL aka_types;
VACUUM FULL akas;
VACUUM FULL crew;
VACUUM FULL genres;
VACUUM FULL names;
VACUUM FULL principals;
VACUUM FULL titles;