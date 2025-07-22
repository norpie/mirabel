use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdatedSession {
    pub title: String,
}
