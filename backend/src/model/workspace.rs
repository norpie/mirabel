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

#[derive(Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Debug, Queryable, Selectable, Insertable, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
            created_at: Utc::now()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Integer)]
pub enum WorkspaceRole {
    Owner = 0,
    Admin = 1,
    Member = 2,
}

impl WorkspaceRole {
    pub fn is_at_least_member(&self) -> bool {
        matches!(self, WorkspaceRole::Member | WorkspaceRole::Admin | WorkspaceRole::Owner)
    }
}

impl FromSql<Integer, Pg> for WorkspaceRole {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(WorkspaceRole::Owner),
            1 => Ok(WorkspaceRole::Admin),
            2 => Ok(WorkspaceRole::Member),
            _ => Err("Unrecognized workspace role".into()),
        }
    }
}

impl ToSql<Integer, Pg> for WorkspaceRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            WorkspaceRole::Owner => out.write_all(&[0])?,
            WorkspaceRole::Admin => out.write_all(&[1])?,
            WorkspaceRole::Member => out.write_all(&[2])?,
        };
        Ok(IsNull::No)
    }
}
