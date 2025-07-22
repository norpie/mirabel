use mirabel_core::dto::api_response::ApiResponse;
use mirabel_core::dto::login_user::LoginUser;
use mirabel_core::dto::page::CursorPageRequest;
use mirabel_core::dto::page::CursorPageResponse;
use mirabel_core::dto::page::PageInfo;
use mirabel_core::dto::page::PageRequest;
use mirabel_core::dto::page::PageResponse;
use mirabel_core::dto::register_user::RegisterUser;
use mirabel_core::dto::session::FullSession;
use mirabel_core::dto::token::AccessToken;
use mirabel_core::dto::workspace::FrontendWorkspace;
use mirabel_core::dto::workspace::NewWorkspace;
use mirabel_core::models::session::Session;
use mirabel_core::models::timeline::AcknowledgmentType;
use mirabel_core::models::timeline::AgentStatus;
use mirabel_core::models::timeline::TimelineEntry;
use mirabel_core::models::timeline::TimelineEntryContent;
use mirabel_core::models::user::User;
use mirabel_core::models::workspace::Workspace;
use mirabel_core::models::workspace::WorkspaceMember;
use mirabel_core::models::workspace::WorkspaceRole;
use actix_web::web;
use actix_web::Scope;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(Scope::new("/users"));
}
