pub mod prelude;

pub mod dto;
pub mod error;
pub mod models;
pub mod schema;
pub mod utils;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use crate::dto::{
        frontend_user::FrontendUser,
        login_user::LoginUser,
        register_user::RegisterUser,
    };
    use crate::models::workspace::{Workspace, WorkspaceRole, WorkspaceMember};
    use ts_rs::TS;

    #[test]
    fn export_bindings() {
        FrontendUser::export().unwrap();
        LoginUser::export().unwrap();
        RegisterUser::export().unwrap();
        Workspace::export().unwrap();
        WorkspaceRole::export().unwrap();
        WorkspaceMember::export().unwrap();
    }
}
