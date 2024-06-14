use bcrypt::verify;
use diesel::{
  deserialize::Queryable, prelude::Insertable, query_dsl::methods::FilterDsl, ExpressionMethods,
  RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::v1::user, establish_connection, schema, schema::sessions, util};

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Session {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub accessed_at: i64,
  pub ip_address: String,
  pub user_agent: String,
}

pub struct SessionMetadata {
  pub ip_address: String,
  pub user_agent: String,
}

pub fn create_user_session(
  auth_user: user::AuthUser,
  metadata: SessionMetadata,
) -> Option<Session> {
  let mut conn = establish_connection();

  let found_user = user::get_user_by_email(&auth_user.email);

  match found_user {
    Some(_) => (),
    None => return None,
  }

  let user_object = found_user.unwrap();

  let valid_password = verify(&auth_user.password, &user_object.password);

  match valid_password {
    Ok(_) => (),
    Err(_) => return None,
  }

  let session = Session {
    id: Uuid::new_v4().to_string(),
    user_id: user_object.id,
    created_at: util::unix_time::unix_ms(),
    accessed_at: util::unix_time::unix_ms(),
    ip_address: metadata.ip_address,
    user_agent: metadata.user_agent,
  };

  let result = diesel::insert_into(schema::sessions::table)
    .values(&session)
    .execute(&mut conn);

  match result {
    Ok(_) => (),
    Err(_) => return None,
  }

  Some(session)
}

pub fn get_user_session_by_id(session_id: &str) -> Option<Session> {
  let mut conn = establish_connection();

  let found_session = schema::sessions::table
    .filter(schema::sessions::id.eq(session_id))
    .first::<Session>(&mut conn);

  match found_session {
    Ok(session) => Some(session),
    Err(_) => None,
  }
}

pub fn get_all_user_sessions(session_id: &str) -> Vec<Session> {
  let mut conn = establish_connection();

  let current_session = get_user_session_by_id(session_id);

  match current_session {
    Some(_) => (),
    None => return Vec::new(),
  }

  let found_sessions = schema::sessions::table
    .filter(schema::sessions::user_id.eq(current_session.unwrap().user_id))
    .load::<Session>(&mut conn);

  match found_sessions {
    Ok(sessions) => sessions,
    Err(_) => Vec::new(),
  }
}
