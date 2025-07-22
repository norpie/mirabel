use std::io::Write;

use chrono::DateTime;
use chrono::Utc;
use diesel::{
    Selectable,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::{Insertable, Queryable},
    serialize::{IsNull, ToSql},
    sql_types::Integer,
};

use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

use crate::utils::id::id;

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize, TS,
)]
#[diesel(table_name = crate::schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(export, export_to = "../../mirabel-web/src/lib/generated/")]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub avatar: Option<String>,
}

impl Workspace {
    pub fn new(name: String) -> Self {
        Self {
            id: id!(),
            name,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            avatar: None,
        }
    }
}

#[derive(
    Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize, TS,
)]
#[diesel(table_name = crate::schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(export, export_to = "../../mirabel-web/src/lib/generated/")]
pub struct WorkspaceMember {
    pub id: String,
    pub workspace_id: String,
    pub user_id: String,
    pub role: WorkspaceRole,
    pub created_at: DateTime<Utc>,
}

impl WorkspaceMember {
    pub fn new(workspace_id: String, user_id: String, role: WorkspaceRole) -> Self {
        Self {
            id: id!(),
            workspace_id,
            user_id,
            role,
            created_at: Utc::now(),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize, TS)]
#[diesel(sql_type = Integer)]
#[ts(export, export_to = "../../mirabel-web/src/lib/generated/")]
pub enum WorkspaceRole {
    Owner = 0,
    Admin = 1,
    Member = 2,
}

impl WorkspaceRole {
    pub fn is_at_least_member(&self) -> bool {
        matches!(
            self,
            WorkspaceRole::Member | WorkspaceRole::Admin | WorkspaceRole::Owner
        )
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(WorkspaceRole::Owner),
            1 => Some(WorkspaceRole::Admin),
            2 => Some(WorkspaceRole::Member),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            WorkspaceRole::Owner => 0,
            WorkspaceRole::Admin => 1,
            WorkspaceRole::Member => 2,
        }
    }
}

impl FromSql<Integer, Pg> for WorkspaceRole {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let value = i32::from_sql(bytes)?;
        let opt_role = WorkspaceRole::from_i32(value);
        match opt_role {
            Some(role) => Ok(role),
            None => Err(format!("Invalid WorkspaceRole value: {value}").into()),
        }
    }
}

impl ToSql<Integer, Pg> for WorkspaceRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let value = self.to_i32();
        out.write_all(&value.to_be_bytes())?;
        Ok(IsNull::No)
    }
}
