use ephemeride_api::api;
use poem::{
  endpoint::StaticFilesEndpoint,
  listener::TcpListener,
  middleware::{Cors, NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "poem=debug");
  }

  // #TODO: make this configurable
  let cors = Cors::new()
    .allow_origin("http://localhost:5173")
    .allow_origin("http://localhost:3000")
    .allow_origin("http://127.0.0.1:5173")
    .allow_origin("http://127.0.0.1:3000");

  let app = Route::new()
    .nest("/api", api::index::endpoint())
    .nest(
      "/",
      StaticFilesEndpoint::new("../www").index_file("index.html"),
    )
    .with((NormalizePath::new(TrailingSlash::Trim), cors));

  Server::new(TcpListener::bind("127.0.0.1:3000"))
    .run(app)
    .await
}
