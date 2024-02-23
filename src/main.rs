use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use std::path::PathBuf;

#[tokio::main]
async fn main() {

    let html_dir = if cfg!(debug_assertions) {
        PathBuf::from("./").join("html")
    } else {
        PathBuf::from("/usr/local/share/personal-website/").join("html")
    };

    let main_page = html_dir.join("site.html");

    let app = Router::new().nest_service("/", ServeDir::new(html_dir).not_found_service(ServeFile::new(main_page)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
