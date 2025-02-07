use crate::{
  error::{error_response, EphemerideError},
  services::{auth, user, UserCredentials},
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
    Ok(session) => Response::builder()
      .status(StatusCode::CREATED)
      .header("Content-Type", "application/json")
      .header("Authorization", &session.id)
      .body(serde_json::to_string(&session).unwrap()),
    Err(error) => error_response(error),
  }
}
