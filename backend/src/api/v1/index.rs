use crate::api::v1;
use poem::{get, patch, post, Route};

#[rustfmt::skip]
pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/user", post(v1::user::create_user)
    .patch(v1::user::update_user)
    .delete(v1::user::delete_user)
    .get(v1::user::get_current_user)
    )

    .at("/user/password", patch(v1::user::update_password))

    .at("/user/categories", get(v1::user::get_user_categories_with_tags))
    
    .at("/auth", post(v1::auth::authenticate_user))

    .at("/auth/config", get(v1::auth::auth_config))
}
