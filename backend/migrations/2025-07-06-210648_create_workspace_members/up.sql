-- Your SQL goes here

CREATE TABLE "workspace_members"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"workspace_id" TEXT NOT NULL,
	"user_id" TEXT NOT NULL,
	"role" INTEGER NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("workspace_id") REFERENCES "workspaces"("id"),
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);


