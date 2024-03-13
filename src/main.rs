use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use awc::Client;

use crate::routes::{in_city, ping, traveling};

pub mod assets;
pub mod drawer;
mod routes;

async fn healthcheck() -> bool {
    let client = Client::default();
    client
        .get("http://localhost:3000/ping")
        .send()
        .await
        .is_ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::args().any(|v| v == "check") {
        let alive = healthcheck().await;
        return if !alive {
            std::process::exit(1)
        } else {
            Ok(())
        };
    }

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
