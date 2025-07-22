use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
}
