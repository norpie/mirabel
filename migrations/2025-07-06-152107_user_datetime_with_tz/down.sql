-- This file should undo anything in `up.sql`


ALTER TABLE "users" DROP COLUMN "created_at";
ALTER TABLE "users" DROP COLUMN "modified_at";
ALTER TABLE "users" ADD COLUMN "created_at" TIMESTAMP NOT NULL;
ALTER TABLE "users" ADD COLUMN "modified_at" TIMESTAMP NOT NULL;

