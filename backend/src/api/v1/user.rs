use crate::services::{auth, invite, user, UserCredentials};
use poem::{
  handler,
  http::StatusCode,
  web::{Json, Path},
  Request, Response,
};

use validator::Validate;

use dotenvy::dotenv;
use std::env;

#[handler]
pub fn create_user(Json(user): Json<user::CreateUser>, request: &Request) -> Response {
  dotenv().ok();

  match user.validate() {
    Ok(_) => (),
    Err(_) => {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Invalid user")
    }
  }

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

  let session = auth::create_user_session(
    UserCredentials {
      email: created_user.email,
      password,
    },
    auth::session_metadata(request),
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
pub fn get_user(id: Path<String>) -> Response {
  let found_user = user::get_user_by_id(&id);

  match found_user {
    Ok(user) => Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "application/json")
      .body(serde_json::to_string(&user).unwrap()),
    Err(user::UserError::NotFound) => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(format!("User with id {} not found", *id)),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(()),
  }
}

#[handler]
pub fn get_current_user(request: &Request) -> Response {
  let session_id = match auth::authorization_from_header(request) {
    Some(session_id) => session_id,
    None => {
      return Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body("Unauthorized");
    }
  };

  let found_user = user::get_current_user(&session_id);

  match found_user {
    Ok(user) => Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "application/json")
      .body(serde_json::to_string(&user).unwrap()),
    Err(user::UserError::NotFound) => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body("User not found"),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(()),
  }
}
