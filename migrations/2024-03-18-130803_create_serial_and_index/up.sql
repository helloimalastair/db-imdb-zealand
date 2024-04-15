-- Your SQL goes here
SELECT SETVAL('titles_tconst_seq', (SELECT MAX(tconst) + 1 FROM titles));
SELECT SETVAL('names_nconst_seq', (SELECT MAX(nconst) + 1+ 1  + 1 FROM names));
CREATE INDEX names_primary_index ON names (primaryname);
CREATE INDEX titles_primary ON titles (primarytitle);
CREATE INDEX titles_original ON titles (originaltitle);