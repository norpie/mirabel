use crate::{
    prelude::*,
    repository::{surrealdb::SurrealDB, Repository},
};

use std::{env, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    middleware::Logger, web::{self, Data}, App, HttpResponse, HttpServer, Scope
};

mod api;
pub(crate) mod extractors;
pub(crate) mod middleware;

pub async fn run(db: Data<Box<dyn Repository>>) -> Result<()> {
    let host = env::var("BACKEND_HOST")?;
    let port: u16 = env::var("BACKEND_PORT")?.parse()?;
    println!("Listening on {}:{}", host, port);
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
