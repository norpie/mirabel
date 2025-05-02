use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdatedSession {
    pub user_title: Option<String>,
}
