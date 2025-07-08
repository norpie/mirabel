-- Your SQL goes here







CREATE TABLE "timeline_entries"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"session_id" TEXT NOT NULL,
	"content" JSONB NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("session_id") REFERENCES "sessions"("id")
);

