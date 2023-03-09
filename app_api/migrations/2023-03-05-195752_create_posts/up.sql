-- Your SQL goes here
CREATE TABLE "posts" (
	"id"	TEXT NOT NULL UNIQUE,
	"title"	TEXT NOT NULL UNIQUE,
	"body"	TEXT NOT NULL,
	"published"	INTEGER NOT NULL,
	PRIMARY KEY("id")
);
