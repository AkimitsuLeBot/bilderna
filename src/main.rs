use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::routes::{in_city, ping, traveling};

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
            .route("/ping", web::get().to(ping))
            .wrap(Logger::new("%r {%T} %s: %{error}o").exclude("/ping"))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
