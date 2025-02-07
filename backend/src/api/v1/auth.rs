use crate::{
  services::{auth, user, UserCredentials},
  util::{
    error::{error_response, EphemerideError},
    response,
  },
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

#[handler]
pub fn authenticate_user(Json(user): Json<user::AuthUser>, request: &Request) -> Response {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return error_response(EphemerideError::BadRequest),
  }

  let session = auth::create_user_session(
    UserCredentials {
      email: String::from(&user.email),
      password: String::from(&user.password),
    },
    auth::session_metadata(request),
  );

  match session {
    Ok(session) => response(StatusCode::CREATED, &session),
    Err(error) => error_response(error),
  }
}
