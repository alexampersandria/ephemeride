use crate::{
  services::{authorize_request, log},
  util::{error::error_response, response, EphemerideError},
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CreateTagRequest {
  name: String,
  color: String,
  category_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct EditTagRequest {
  id: String,
  name: String,
  color: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteTagRequest {
  id: String,
}

#[handler]
pub fn create_tag(Json(tag): Json<CreateTagRequest>, request: &Request) -> Response {
  let session = match authorize_request(request) {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let created_tag = log::create_tag(log::CreateTag {
    name: tag.name,
    color: tag.color,
    category_id: tag.category_id,
    user_id: session.user_id,
  });

  match created_tag {
    Ok(created_tag) => response(StatusCode::CREATED, &created_tag),
    Err(error) => error_response(error),
  }
}

#[handler]
pub fn edit_tag(Json(tag): Json<EditTagRequest>, request: &Request) -> Response {
  let session = match authorize_request(request) {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let edited_tag = log::edit_tag(log::EditTag {
    id: tag.id,
    name: tag.name,
    color: tag.color,
    user_id: session.user_id,
  });

  match edited_tag {
    Ok(edited_tag) => response(StatusCode::OK, &edited_tag),
    Err(error) => error_response(error),
  }
}

#[handler]
pub fn delete_tag(Json(tag): Json<DeleteTagRequest>, request: &Request) -> Response {
  let session = match authorize_request(request) {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let deleted_tag = match log::delete_tag(&tag.id, &session.user_id) {
    Ok(deleted) => deleted,
    Err(error) => return error_response(error),
  };

  match deleted_tag {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(EphemerideError::TagNotFound),
  }
}
