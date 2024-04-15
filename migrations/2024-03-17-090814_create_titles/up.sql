-- Your SQL goes here
CREATE TABLE "titles"(
	"tconst" SERIAL NOT NULL PRIMARY KEY,
	"titletype" VARCHAR,
	"primarytitle" VARCHAR,
	"originaltitle" VARCHAR,
	"isadult" BOOL,
	"startyear" INT4,
	"endyear" INT4,
	"runtimeminutes" INT4
);
CREATE TABLE "crew"(
	"tconst" INT4 NOT NULL,
	"nconst" INT4 NOT NULL,
	"isdirector" BOOL NOT NULL,
	PRIMARY KEY("tconst", "nconst", "isdirector")
);

CREATE TABLE "genres"(
	"tconst" INT4 NOT NULL,
	"genre" VARCHAR NOT NULL,
	PRIMARY KEY("tconst", "genre")
);