use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken<'a> {
    access_token: &'a str,
}

impl<'a> From<&'a str> for AccessToken<'a> {
    fn from(access_token: &'a str) -> Self {
        Self { access_token }
    }
}
