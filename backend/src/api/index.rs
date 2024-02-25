use crate::api::v1;
use poem::Route;

pub fn v1_endpoint() -> poem::Route {
  Route::new().nest("/v1", v1::endpoint())
}

pub fn endpoint() -> poem::Route {
  Route::new().nest("/", v1_endpoint())
}
