use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../mirabel-web/src/lib/generated/")]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
