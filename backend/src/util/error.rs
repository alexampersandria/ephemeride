use crate::util::response::response;
use poem::{http::StatusCode, Response};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum EphemerideError {
  Unauthorized,
  DatabaseError,
  InternalServerError,
  UserNotFound,
  InviteNotFound,
  SessionNotFound,
  EmailAlreadyInUse,
  InvalidPassword,
  InviteUsed,
  BadRequest,
}

#[derive(Serialize)]
struct ErrorBody {
  code: EphemerideError,
  message: String,
}

fn error_message(error: EphemerideError) -> String {
  match error {
    EphemerideError::Unauthorized => "Unauthorized",
    EphemerideError::UserNotFound => "User not found",
    EphemerideError::InviteNotFound => "Invite not found",
    EphemerideError::SessionNotFound => "Session not found",
    EphemerideError::EmailAlreadyInUse => "Email already in use",
    EphemerideError::InvalidPassword => "Invalid password",
    EphemerideError::InviteUsed => "Invite already used",
    _ => "",
  }
  .to_string()
}

fn status_code(error: EphemerideError) -> StatusCode {
  match error {
    EphemerideError::Unauthorized => StatusCode::UNAUTHORIZED,
    EphemerideError::UserNotFound => StatusCode::NOT_FOUND,
    EphemerideError::InviteNotFound => StatusCode::NOT_FOUND,
    EphemerideError::SessionNotFound => StatusCode::NOT_FOUND,
    EphemerideError::EmailAlreadyInUse => StatusCode::CONFLICT,
    EphemerideError::InvalidPassword => StatusCode::UNAUTHORIZED,
    EphemerideError::InviteUsed => StatusCode::CONFLICT,
    EphemerideError::BadRequest => StatusCode::BAD_REQUEST,
    _ => StatusCode::INTERNAL_SERVER_ERROR,
  }
}

fn error_body(error: EphemerideError) -> ErrorBody {
  ErrorBody {
    code: error,
    message: error_message(error),
  }
}

pub fn error_response(error: EphemerideError) -> Response {
  response(status_code(error), &error_body(error))
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn test_error_response() {
    let error = EphemerideError::Unauthorized;
    let response = error_response(error);
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
  }
}
