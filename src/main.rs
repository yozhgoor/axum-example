use anyhow::Result;
use axum::{response::Html, routing::get, Router};
use lazy_static::lazy_static;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

const ADDRESS: &str = "127.0.0.1:3000";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => panic!("cannot compile templates: {e}"),
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(ADDRESS)
        .await
        .expect("can bind address");

    println!("Listening on http://{}", ADDRESS);

    axum::serve(listener, app).await.expect("can serve app");

    Ok(())
}

async fn index() -> Html<String> {
    Html(
        TEMPLATES
            .render("index.html", &Context::new())
            .expect("can render template"),
    )
}
