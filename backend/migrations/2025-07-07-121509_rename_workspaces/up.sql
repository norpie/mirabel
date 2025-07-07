-- Your SQL goes here






CREATE TABLE "sessions"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"workspace_id" TEXT NOT NULL,
	"user_id" TEXT NOT NULL,
	"title" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"modified_at" TIMESTAMPTZ NOT NULL,
	"archived" BOOL NOT NULL,
	FOREIGN KEY ("workspace_id") REFERENCES "workspaces"("id"),
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

