-- Your SQL goes here
CREATE TABLE "names"(
	"nconst" SERIAL NOT NULL PRIMARY KEY,
	"primaryname" VARCHAR NOT NULL,
	"birthyear" INT4,
	"deathyear" INT4
);