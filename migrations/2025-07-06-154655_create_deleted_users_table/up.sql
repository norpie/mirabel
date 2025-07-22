-- Your SQL goes here



CREATE TABLE "deleted_users"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"deleted_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("id") REFERENCES "users"("id")
);

