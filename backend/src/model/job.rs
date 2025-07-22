use std::io::Write;

use chrono::{DateTime, Utc};
use diesel::{
    Selectable,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::{Insertable, Queryable},
    serialize::{IsNull, ToSql},
    sql_types::Integer,
};
use serde::{Deserialize, Serialize};

use crate::driver::id::id;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub enum JobType {
    Router = 4,              // Same as the agent of the same name, used to route user input to the
                             // appropriate agent
    TitleGeneration = 0,     // Generate a title for the session
    SessionSummary = 1,      // Summarize the session
    ContentExtraction = 3, // Extract content from user input (secret, email, code snippet, etc.)
}

impl JobType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            4 => Some(JobType::Router),
            0 => Some(JobType::TitleGeneration),
            1 => Some(JobType::SessionSummary),
            // 2 => Some(JobType::CategorizeUserInput),
            3 => Some(JobType::ContentExtraction),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        match self {
            JobType::Router => 4,
            JobType::TitleGeneration => 0,
            JobType::SessionSummary => 1,
            // JobType::CategorizeUserInput => 2,
            JobType::ContentExtraction => 3,
        }
    }
}

impl FromSql<Integer, Pg> for JobType {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        let opt_job_type = JobType::from_i32(value);
        match opt_job_type {
            Some(job_type) => Ok(job_type),
            None => Err(format!("Invalid JobType value: {value}").into()),
        }
    }
}

impl ToSql<Integer, Pg> for JobType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let value = self.to_i32();
        out.write_all(&value.to_be_bytes())?;
        Ok(IsNull::No)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub enum JobStatus {
    Pending = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}

impl JobStatus {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(JobStatus::Pending),
            1 => Some(JobStatus::InProgress),
            2 => Some(JobStatus::Completed),
            3 => Some(JobStatus::Failed),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        match self {
            JobStatus::Pending => 0,
            JobStatus::InProgress => 1,
            JobStatus::Completed => 2,
            JobStatus::Failed => 3,
        }
    }
}

impl FromSql<Integer, Pg> for JobStatus {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        let opt_status = JobStatus::from_i32(value);
        match opt_status {
            Some(status) => Ok(status),
            None => Err(format!("Invalid JobStatus value: {value}").into()),
        }
    }
}

impl ToSql<Integer, Pg> for JobStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let value = self.to_i32();
        out.write_all(&value.to_be_bytes())?;
        Ok(IsNull::No)
    }
}

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: String,
    pub session_id: String,
    pub parent_job_id: Option<String>,
    pub job_type: JobType,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
}

impl Job {
    pub fn new(
        session_id: String,
        parent_job_id: Option<String>,
        job_type: JobType,
        status: JobStatus,
    ) -> Self {
        Self {
            id: id!(),
            session_id,
            parent_job_id,
            job_type,
            status,
            created_at: Utc::now(),
        }
    }
}
