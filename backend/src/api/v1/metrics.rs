use crate::{
  services::user,
  util::{error_response, response, unix_ms, EphemerideError},
};
use poem::{handler, http::StatusCode, Response};

#[derive(Debug, serde::Serialize)]
pub struct MetricsResponse {
  pub total_users: i64,
  pub active_1h: i64,
  pub active_24h: i64,
  pub active_7d: i64,
  pub active_30d: i64,
}

#[handler]
pub fn metrics() -> Response {
  let total_users = match user::user_count() {
    Ok(count) => count,
    Err(_) => return error_response(EphemerideError::InternalServerError),
  };
  let active_1h = match user::active_user_count(unix_ms() - 60 * 60 * 1000) {
    Ok(count) => count,
    Err(_) => return error_response(EphemerideError::InternalServerError),
  };
  let active_24h = match user::active_user_count(unix_ms() - 24 * 60 * 60 * 1000) {
    Ok(count) => count,
    Err(_) => return error_response(EphemerideError::InternalServerError),
  };
  let active_7d = match user::active_user_count(unix_ms() - 7 * 24 * 60 * 60 * 1000) {
    Ok(count) => count,
    Err(_) => return error_response(EphemerideError::InternalServerError),
  };
  let active_30d = match user::active_user_count(unix_ms() - 30 * 24 * 60 * 60 * 1000) {
    Ok(count) => count,
    Err(_) => return error_response(EphemerideError::InternalServerError),
  };

  response(
    StatusCode::OK,
    &MetricsResponse {
      total_users,
      active_1h,
      active_24h,
      active_7d,
      active_30d,
    },
  )
}
