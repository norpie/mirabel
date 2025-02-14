use std::sync::Arc;

use crate::prelude::*;

use async_trait::async_trait;
use users::UserRepository;

pub(crate) mod surrealdb;

pub(crate) mod users;
pub(crate) mod workspaces;

#[async_trait]
pub trait Repository: Send + Sync + UserRepository {}
