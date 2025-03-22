use std::sync::Arc;

use crate::prelude::*;

use async_trait::async_trait;
use avatars::AvatarRepository;
use sessions::SessionRepository;
use users::UserRepository;
use workspaces::WorkspaceRepository;

pub(crate) mod surrealdb;

pub(crate) mod avatars;
pub(crate) mod sessions;
pub(crate) mod users;
pub(crate) mod workspaces;

#[async_trait]
pub trait Repository:
    Send + Sync + AvatarRepository + UserRepository + WorkspaceRepository + SessionRepository
{
}

#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::web::Data;

    use crate::{
        dto::page::PageRequest,
        model::{
            user::{NewUser, UpdatedUser},
            workspace::{NewWorkspace, UpdatedWorkspace},
        },
        repository::surrealdb::SurrealDB,
    };

    use super::Repository;

    async fn test_repository(repo: Data<Box<dyn Repository>>) {
        let username = "test".to_string();
        let email = "test@gmail.com".to_string();
        let password = "password".to_string();

        let mut user = repo
            .create_user(NewUser::new(
                username.clone(),
                email.clone(),
                password.clone(),
            ))
            .await
            .unwrap();

        let found_by_id = repo
            .get_user_by_id(user.id().to_string())
            .await
            .unwrap()
            .unwrap();

        let found_by_email = repo
            .get_user_by_email(email.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(user, found_by_id);
        assert_eq!(user, found_by_email);

        let all_users = repo.get_all_users(PageRequest::default()).await.unwrap();
        assert_eq!(all_users.len(), 1);

        let new_user = repo
            .update_user(
                user.id().to_string(),
                UpdatedUser {
                    email: Some("test2@gmail.com".to_string()),
                    password: Some(user.password().to_string()),
                },
            )
            .await
            .unwrap();

        assert_eq!(new_user.email(), "test2@gmail.com");

        let new_workspace = NewWorkspace {
            name: "Test Workspace".to_string(),
        };

        let created_workspace = repo
            .create_workspace(user.id().to_string(), new_workspace.clone())
            .await
            .unwrap();

        let found_workspace = repo
            .get_workspace_by_id(created_workspace.id().to_string())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found_workspace.name(), "Test Workspace");

        let updated_workspace = UpdatedWorkspace {
            name: Some("Updated Test Workspace".to_string()),
        };

        let updated_workspace_result = repo
            .update_workspace(created_workspace.id().to_string(), updated_workspace)
            .await
            .unwrap();

        assert_eq!(updated_workspace_result.name(), "Updated Test Workspace");

        let avatar_url = "https://avatars.githubusercontent.com/u/46564751?v=4";

        // Test setting an avatar
        repo.set_avatar(
            "user".to_string(),
            user.id().to_string(),
            avatar_url.to_string(),
        )
        .await
        .unwrap();

        // Test getting an avatar
        let retrieved_avatar = repo
            .get_avatar("user".to_string(), user.id().to_string())
            .await
            .unwrap();
        assert_eq!(retrieved_avatar, Some(avatar_url.to_string()));

        // Test deleting an avatar
        repo.delete_avatar("user".to_string(), user.id().to_string())
            .await
            .unwrap();

        repo.delete_workspace(updated_workspace_result.id().to_string())
            .await
            .unwrap();

        repo.delete_user(user.id().to_string()).await.unwrap();
    }

    #[tokio::test]
    async fn test_surrealdb() {
        env::set_var("DATABASE_HOST", "localhost");
        env::set_var("DATABASE_PORT", "8000");
        env::set_var("DATABASE_NS", "test");
        env::set_var("DATABASE_DB", "test");
        env::set_var("DATABASE_USER", "root");
        env::set_var("DATABASE_PASS", "root");
        let test_db = SurrealDB::setup().await.unwrap();
        test_repository(test_db).await;
    }
}
