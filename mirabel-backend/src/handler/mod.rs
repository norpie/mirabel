use crate::prelude::*;

use crate::service::auth::AuthService;
use crate::service::sessions::SessionService;
use crate::service::users::UserService;
use crate::service::workspaces::WorkspaceService;

use crate::driver::llm::ollama::Ollama;

use std::env;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::Data;
use actix_web::{Result as ActixResult, HttpRequest};

use deadpool_diesel::postgres::Pool;
use log::info;

mod api;
pub(crate) mod extractors;
pub(crate) mod middleware;

// SPA fallback handler for frontend routes
async fn spa_fallback(_req: HttpRequest) -> ActixResult<NamedFile> {
    let path = std::path::Path::new("../mirabel-web/build/200.html");
    Ok(NamedFile::open(path)?)
}

pub async fn run(db: Data<Pool>, llm: Data<Ollama>) -> Result<()> {
    let host = env::var("BACKEND_HOST")?;
    let port: u16 = env::var("BACKEND_PORT")?.parse()?;

    let auth_service = Data::new(AuthService::from(db.clone())?);
    let user_service = Data::new(UserService::from(db.clone())?);
    let workspace_service = Data::new(WorkspaceService::from(db.clone())?);
    let session_service = Data::new(SessionService::from(db.clone(), llm.clone())?);

    info!("Listening on {host}:{port}");
    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(db.clone())
            .app_data(llm.clone())
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .app_data(session_service.clone())
            .app_data(workspace_service.clone())
            .wrap(cors)
            .wrap(logger)
            .configure(api::scope)
            // Serve static files from the web build directory
            .service(
                Files::new("/", "../mirabel-web/build/")
                    .index_file("200.html")
                    .default_handler(web::get().to(spa_fallback))
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
