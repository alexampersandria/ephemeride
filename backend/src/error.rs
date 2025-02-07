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

fn error_body(error: EphemerideError) -> String {
  match serde_json::to_string(&ErrorBody {
    code: error,
    message: error_message(error),
  }) {
    Ok(body) => body,
    Err(_) => "".to_string(),
  }
}

pub fn error_response(error: EphemerideError) -> Response {
  Response::builder()
    .status(status_code(error))
    .header("Content-Type", "application/json")
    .body(error_body(error))
}
