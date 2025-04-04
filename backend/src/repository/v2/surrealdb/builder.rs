use crate::prelude::*;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Database, Namespace, Root},
    Surreal,
};

use super::SurrealDB;

// Auth types remain the same
pub enum SurrealDBAuth<'a> {
    Root {
        username: &'a str,
        password: &'a str,
    },
    NamespaceUser {
        namespace: &'a str,
        username: &'a str,
        password: &'a str,
    },
    DatabaseUser {
        namespace: &'a str,
        database: &'a str,
        username: &'a str,
        password: &'a str,
    },
}

// Define state markers
pub struct NonAuthed;
pub struct RootAuthed<'a> {
    username: &'a str,
    password: &'a str,
}
pub struct NamespaceAuthed<'a> {
    namespace: &'a str,
    username: &'a str,
    password: &'a str,
}
pub struct DatabaseAuthed<'a> {
    namespace: &'a str,
    database: &'a str,
    username: &'a str,
    password: &'a str,
}

// Generic builder with state type parameter
pub struct SurrealDBBuilder<'a, S> {
    url: &'a str,
    state: S,
}

// Specialized builder variants with additional fields
pub struct RootAuthedBuilder<'a> {
    url: &'a str,
    state: RootAuthed<'a>,
}

pub struct RootWithNsBuilder<'a> {
    url: &'a str,
    state: RootAuthed<'a>,
    ns: &'a str,
}

pub struct RootWithNsDbBuilder<'a> {
    url: &'a str,
    state: RootAuthed<'a>,
    ns: &'a str,
    db: &'a str,
}

pub struct NamespaceAuthedBuilder<'a> {
    url: &'a str,
    state: NamespaceAuthed<'a>,
}

pub struct NamespaceWithDbBuilder<'a> {
    url: &'a str,
    state: NamespaceAuthed<'a>,
    db: &'a str,
}

// Initial state implementation
impl<'a> SurrealDBBuilder<'a, NonAuthed> {
    pub fn new(url: &'a str) -> Self {
        SurrealDBBuilder {
            url,
            state: NonAuthed,
        }
    }

    pub fn with_root(
        self,
        username: &'a str,
        password: &'a str,
    ) -> RootAuthedBuilder<'a> {
        RootAuthedBuilder {
            url: self.url,
            state: RootAuthed { username, password },
        }
    }

    pub fn with_namespace_user(
        self,
        namespace: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> NamespaceAuthedBuilder<'a> {
        NamespaceAuthedBuilder {
            url: self.url,
            state: NamespaceAuthed {
                namespace,
                username,
                password,
            },
        }
    }

    pub fn with_database_user(
        self,
        namespace: &'a str,
        database: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> SurrealDBBuilder<'a, DatabaseAuthed<'a>> {
        SurrealDBBuilder {
            url: self.url,
            state: DatabaseAuthed {
                namespace,
                database,
                username,
                password,
            },
        }
    }
}

// Root authenticated state implementation
impl<'a> RootAuthedBuilder<'a> {
    pub fn use_ns(
        self,
        namespace: &'a str,
    ) -> RootWithNsBuilder<'a> {
        RootWithNsBuilder {
            url: self.url,
            state: self.state,
            ns: namespace,
        }
    }
}

// Root with namespace implementation
impl<'a> RootWithNsBuilder<'a> {
    pub fn use_db(
        self,
        database: &'a str,
    ) -> RootWithNsDbBuilder<'a> {
        RootWithNsDbBuilder {
            url: self.url,
            state: self.state,
            ns: self.ns,
            db: database,
        }
    }
}

// Root with namespace and database implementation
impl<'a> RootWithNsDbBuilder<'a> {
    pub async fn build(self) -> Result<SurrealDB> {
        let connection = Surreal::new::<Ws>(self.url).await?;
        connection.signin(Root {
            username: self.state.username,
            password: self.state.password
        }).await?;

        connection.use_ns(self.ns).await?;
        connection.use_db(self.db).await?;

        Ok(connection.into())
    }
}

// Namespace authenticated state implementation
impl<'a> NamespaceAuthedBuilder<'a> {
    pub fn use_db(
        self,
        database: &'a str,
    ) -> NamespaceWithDbBuilder<'a> {
        NamespaceWithDbBuilder {
            url: self.url,
            state: self.state,
            db: database,
        }
    }
}

// Namespace with database implementation
impl<'a> NamespaceWithDbBuilder<'a> {
    pub async fn build(self) -> Result<SurrealDB> {
        let connection = Surreal::new::<Ws>(self.url).await?;
        connection.signin(Namespace {
            namespace: self.state.namespace,
            username: self.state.username,
            password: self.state.password,
        }).await?;

        // Always use the namespace we authenticated with
        connection.use_ns(self.state.namespace).await?;
        connection.use_db(self.db).await?;

        Ok(connection.into())
    }
}

// Database authenticated state implementation
impl<'a> SurrealDBBuilder<'a, DatabaseAuthed<'a>> {
    pub async fn build(self) -> Result<SurrealDB> {
        let connection = Surreal::new::<Ws>(self.url).await?;
        connection.signin(Database {
            namespace: self.state.namespace,
            database: self.state.database,
            username: self.state.username,
            password: self.state.password,
        }).await?;

        // Always use the namespace and database we authenticated with
        connection.use_ns(self.state.namespace).await?;
        connection.use_db(self.state.database).await?;

        Ok(connection.into())
    }
}
