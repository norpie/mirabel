use chrono::DateTime;
use chrono::Utc;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};

use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::prompt_evaluations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PromptEvaluation {
    pub id: String,
    pub job_id: String,
    pub prompt_token_count: i32,
    pub response_token_count: i32,
    pub evaluation_start: DateTime<Utc>,
    pub evaluation_end: DateTime<Utc>,
}
