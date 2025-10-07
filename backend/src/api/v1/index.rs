use crate::api::v1;
use poem::{get, post, Route};

#[rustfmt::skip]
pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/user", post(v1::user::create_user).get(v1::user::get_current_user))
    .at("/auth", post(v1::auth::authenticate_user))
    .at("/auth/config", get(v1::auth::auth_config))
}
