use poem::{get, handler, Route};

#[rustfmt::skip]
pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/", get(package_version))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
