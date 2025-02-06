use crate::services::{invite, session, user, UserCredentials};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

use dotenvy::dotenv;
use std::env;

#[handler]
pub fn create_user(Json(user): Json<user::CreateUser>, request: &Request) -> Response {
  dotenv().ok();

  // validate user object, if invalid return 400 bad request
  match user.validate() {
    Ok(_) => (),
    Err(_) => {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid user")
    }
  }

  // check if INVITE_REQUIRED .env variable is set to true, if so check if invite is valid
  if env::var("INVITE_REQUIRED").unwrap_or("false".to_string()) == "true" {
    match &user.invite {
      Some(invite) => {
        let found_invite = invite::get_invite(invite);
        match found_invite {
          Ok(invite) => {
            if invite.used {
              return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("Invite has already been used");
            }
          }
          Err(_) => {
            return Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .body("Invalid invite");
          }
        }
      }
      // if invite is not provided but required in .env return 401 unauthorized
      None => {
        return Response::builder()
          .status(StatusCode::UNAUTHORIZED)
          .body("Invite is required");
      }
    }
  }

  let password = user.password.clone();
  let created_user = user::create_user(user);

  let created_user = match created_user {
    Ok(user) => user,
    Err(_) => {
      return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Failed to create user");
    }
  };

  let session = session::create_user_session(
    UserCredentials {
      email: String::from(&created_user.email),
      password: String::from(password),
    },
    session::SessionMetadata {
      ip_address: request.remote_addr().to_string(),
      user_agent: request
        .header("user-agent")
        .unwrap_or("unknown")
        .to_string(),
    },
  );

  match session {
    Ok(session) => Response::builder()
      .status(StatusCode::CREATED)
      .header("Content-Type", "application/json")
      .header("Authorization", &session.id)
      .body(serde_json::to_string(&session).unwrap()),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(()),
  }
}

#[handler]
pub fn auth_user(Json(user): Json<user::AuthUser>, request: &Request) -> Response {
  // validate user object, if invalid return 400 bad request
  match user.validate() {
    Ok(_) => (),
    Err(_) => {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid user")
    }
  }

  let session = session::create_user_session(
    UserCredentials {
      email: String::from(&user.email),
      password: String::from(&user.password),
    },
    session::SessionMetadata {
      ip_address: request.remote_addr().to_string(),
      user_agent: request
        .header("user-agent")
        .unwrap_or("unknown")
        .to_string(),
    },
  );

  match session {
    Ok(session) => Response::builder()
      .status(StatusCode::CREATED)
      .header("Content-Type", "application/json")
      .header("Authorization", &session.id)
      .body(serde_json::to_string(&session).unwrap()),
    Err(session::SessionError::NotFound) => Response::builder()
      .status(StatusCode::UNAUTHORIZED)
      .body("User not found"),
    Err(session::SessionError::InvalidPassword) => Response::builder()
      .status(StatusCode::UNAUTHORIZED)
      .body("Invalid password"),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(()),
  }
}
