use crate::{
  services::{auth, authorize_request, invite, user, UserCredentials},
  util::{
    error::{error_response, EphemerideError},
    response,
  },
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

use dotenvy::dotenv;
use std::env;

#[handler]
pub fn create_user(Json(user): Json<user::CreateUser>, request: &Request) -> Response {
  dotenv().ok();

  match user.validate() {
    Ok(_) => (),
    Err(_) => return error_response(EphemerideError::BadRequest),
  }

  if env::var("INVITE_REQUIRED").unwrap_or("false".to_string()) == "true" {
    match &user.invite {
      Some(invite) => match invite::use_invite(invite) {
        Ok(_) => (),
        Err(_) => return error_response(EphemerideError::InviteNotFound),
      },
      None => return error_response(EphemerideError::InviteNotFound),
    }
  }

  let password = user.password.clone();
  let created_user = match user::create_user(user) {
    Ok(user) => user,
    Err(error) => return error_response(error),
  };

  let session = auth::create_user_session(
    UserCredentials {
      email: created_user.email,
      password,
    },
    auth::session_metadata(request),
  );

  match session {
    Ok(session) => response(StatusCode::CREATED, &session),
    Err(error) => error_response(error),
  }
}

#[handler]
pub fn get_current_user(request: &Request) -> Response {
  let session = match authorize_request(request) {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let user = user::get_user(&session.user_id);

  match user {
    Ok(user) => response(StatusCode::OK, &user),
    Err(error) => error_response(error),
  }
}
