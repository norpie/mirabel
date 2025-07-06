-- This file should undo anything in `up.sql`
CREATE TABLE "_welds_migrations"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"when_applied" INT8 NOT NULL,
	"rollback_sql" TEXT NOT NULL
);

ALTER TABLE "auth_options" DROP COLUMN "two_factor_encoded";
ALTER TABLE "auth_options" ADD COLUMN "two_factor_encoded" TEXT NOT NULL;


CREATE TABLE "deleted_users"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"user_id" TEXT NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);


