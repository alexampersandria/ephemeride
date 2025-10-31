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
    
    .at("/category", post(v1::category::create_category))
    .at("/category/:id", patch(v1::category::edit_category)
    .delete(v1::category::delete_category))

    .at("/tag", post(v1::tag::create_tag))
    .at("/tag/:id", patch(v1::tag::edit_tag)
    .delete(v1::tag::delete_tag))

    .at("/entry", post(v1::entry::create_entry))
    .at("/entry/:id", patch(v1::entry::edit_entry)
    .delete(v1::entry::delete_entry))
    .at("/entries/:from_date/:to_date", get(v1::entries::get_entries))

    .at("/auth", post(v1::auth::authenticate_user))

    .at("/auth/config", get(v1::auth::auth_config))
}
