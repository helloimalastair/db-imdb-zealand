-- Your SQL goes here
CREATE TABLE "akas"(
	"tconst" INT4 NOT NULL,
	"ordering" INT4 NOT NULL,
	"title" VARCHAR,
	"region" VARCHAR,
	"language" VARCHAR,
	"is_original_title" BOOL,
	PRIMARY KEY("tconst", "ordering")
);
CREATE TABLE "aka_types"(
	"tconst" INT4 NOT NULL,
	"ordering" INT4 NOT NULL,
	"type_name" VARCHAR NOT NULL,
	PRIMARY KEY("tconst", "ordering", "type_name")
);
CREATE TABLE "aka_attributes"(
	"tconst" INT4 NOT NULL,
	"ordering" INT4 NOT NULL,
	"attribute" VARCHAR NOT NULL,
	PRIMARY KEY("tconst", "ordering", "attribute")
);