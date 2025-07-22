-- Your SQL goes here




CREATE TABLE "workspaces"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"avatar" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	"modified_at" TIMESTAMPTZ NOT NULL
);

