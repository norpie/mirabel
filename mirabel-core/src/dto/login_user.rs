use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../frontend/src/lib/generated/")]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
