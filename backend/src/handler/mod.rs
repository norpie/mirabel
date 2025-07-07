use crate::{
    prelude::*,
    service::{
        auth::AuthService, sessions::SessionService, users::UserService,
        workspaces::WorkspaceService,
    },
};

use std::env;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    middleware::Logger,
    web::{self, Data},
};
use deadpool_diesel::postgres::Pool;
use log::info;

mod api;
pub(crate) mod extractors;
pub(crate) mod middleware;

pub async fn run(db: Data<Pool>) -> Result<()> {
    let host = env::var("BACKEND_HOST")?;
    let port: u16 = env::var("BACKEND_PORT")?.parse()?;

    let auth_service = Data::new(AuthService::from(db.clone())?);
    let user_service = Data::new(UserService::from(db.clone())?);
    let workspace_service = Data::new(WorkspaceService::from(db.clone())?);
    let session_service = Data::new(SessionService::from(db.clone())?);

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
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .app_data(session_service.clone())
            .app_data(workspace_service.clone())
            .wrap(cors)
            .wrap(logger)
            .configure(api::scope)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
