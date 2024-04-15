-- Your SQL goes here
CREATE TABLE "principals"(
	"tconst" INT4 NOT NULL,
	"ordering" INT4 NOT NULL,
	"nconst" INT4 NOT NULL,
	"category" VARCHAR NOT NULL,
	"job" VARCHAR,
	"characters" VARCHAR,
	PRIMARY KEY("tconst", "ordering", "nconst")
);