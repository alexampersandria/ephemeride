use crate::api::v1;
use poem::{get, handler, post, Route};

#[rustfmt::skip]
pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/", get(package_version))
    .at("/user", post(v1::user::create_user))
    .at("/auth", post(v1::user::auth_user))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
