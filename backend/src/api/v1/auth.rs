use crate::{
  errors::EphemerideError,
  services::{auth, user, UserCredentials},
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

#[handler]
pub fn authenticate_user(Json(user): Json<user::AuthUser>, request: &Request) -> Response {
  match user.validate() {
    Ok(_) => (),
    Err(_) => {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid user")
    }
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
    Err(EphemerideError::UserNotFound) => Response::builder()
      .status(StatusCode::UNAUTHORIZED)
      .body("User not found"),
    Err(EphemerideError::InvalidPassword) => Response::builder()
      .status(StatusCode::UNAUTHORIZED)
      .body("Invalid password"),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(()),
  }
}
