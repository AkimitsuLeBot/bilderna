use actix_web::{HttpServer, web, App, middleware::Logger};

use crate::routes::{traveling, in_city};

pub mod assets;
pub mod drawer;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .route("/traveling", web::post().to(traveling))
            .route("/in_city", web::post().to(in_city))
            .wrap(Logger::new("%r {%T} %s: %{error}o"))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}