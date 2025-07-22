-- Your SQL goes here








CREATE TABLE "jobs"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"session_id" TEXT NOT NULL,
	"parent_job_id" TEXT,
	"job_type" INT4 NOT NULL,
	"status" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("session_id") REFERENCES "sessions"("id")
);

CREATE TABLE "prompt_evaluations"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"job_id" TEXT NOT NULL,
	"prompt_token_count" INT4 NOT NULL,
	"response_token_count" INT4 NOT NULL,
	"evaluation_start" TIMESTAMPTZ NOT NULL,
	"evaluation_end" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("job_id") REFERENCES "jobs"("id")
);

