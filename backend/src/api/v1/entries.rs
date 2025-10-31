use crate::{
  services::{authorize_request, log},
  util::{error::error_response, response},
};
use poem::{handler, http::StatusCode, web::Path, Request, Response};

#[handler]
pub fn get_entries(
  Path((from_date, to_date)): Path<(String, String)>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request) {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let entries = log::get_entries_in_range(&from_date, &to_date, &session.user_id);

  match entries {
    Ok(entries) => response(StatusCode::OK, &entries),
    Err(error) => error_response(error),
  }
}
