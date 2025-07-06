-- Your SQL goes here
DROP TABLE IF EXISTS "_welds_migrations";
ALTER TABLE "auth_options" DROP COLUMN "two_factor_encoded";
ALTER TABLE "auth_options" ADD COLUMN "two_factor_encoded" TEXT;


DROP TABLE IF EXISTS "deleted_users";

