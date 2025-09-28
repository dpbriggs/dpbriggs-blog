#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

mod blog;
mod context;
mod routes;

use axum::Router;
use axum_template::engine::Engine;
use tera::Tera;
use tower_http::services::ServeDir;

use crate::routes::get_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let tera = Tera::new("templates/**/*.tera").unwrap();
    let engine = Engine::from(tera);

    let app = Router::new()
        .merge(get_routes())
        .nest_service("/static", ServeDir::new("static"))
        .layer(axum::extract::Extension(engine));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
