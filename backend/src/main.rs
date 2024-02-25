use dotenvy::dotenv;
use std::env;

use ephemeride_backend::api;
use poem::{
  endpoint::StaticFilesEndpoint,
  listener::TcpListener,
  middleware::{Cors, NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  let port = env::var("PORT").unwrap_or("3000".to_string());

  let cors = Cors::new()
    .allow_origin("http://localhost:5173")
    .allow_origin("http://127.0.0.1:5173")
    .allow_origin(format!("http://localhost:{}", port))
    .allow_origin(format!("http://127.0.0.1:{}", port));

  let app = Route::new()
    .nest("/api", api::index::endpoint())
    .nest(
      "/",
      StaticFilesEndpoint::new("../www").index_file("index.html"),
    )
    .with((NormalizePath::new(TrailingSlash::Trim), cors));

  println!("listening on port {}", port);

  Server::new(TcpListener::bind(format!("127.0.0.1:{}", port)))
    .run(app)
    .await
}
