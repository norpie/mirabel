use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum SDBError {
    #[error("Generic surrealdb error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("Not found recent update for `{0}`")]
    NotFoundRecentUpdate(String),
    #[error("Not found")]
    NotFound,
}

#[derive(Deserialize)]
struct SDBCount(Vec<Count>);

impl SDBCount {
    fn count(&self) -> u64 {
        self.0.first().map_or(0, |c| c.count)
    }
}

#[derive(Deserialize)]
struct Count {
    count: u64,
}
