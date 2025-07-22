-- Your SQL goes here


ALTER TABLE "users" DROP COLUMN "created_at";
ALTER TABLE "users" DROP COLUMN "modified_at";
ALTER TABLE "users" ADD COLUMN "created_at" TIMESTAMPTZ NOT NULL;
ALTER TABLE "users" ADD COLUMN "modified_at" TIMESTAMPTZ NOT NULL;

