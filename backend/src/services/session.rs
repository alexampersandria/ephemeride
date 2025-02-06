use bcrypt::verify;
use diesel::{
  deserialize::Queryable, prelude::Insertable, query_dsl::methods::FilterDsl, ExpressionMethods,
  RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{establish_connection, schema, schema::sessions, services::user, util};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCredentials {
  pub email: String,
  pub password: String,
}

pub fn create_user_session(
  user_credentials: UserCredentials,
  metadata: SessionMetadata,
) -> Option<Session> {
  let mut conn = establish_connection();

  let found_user = user::get_user_by_email(&user_credentials.email);

  match found_user {
    Some(_) => (),
    None => return None,
  }

  let user_object = found_user.unwrap();

  let valid_password = verify(&user_credentials.password, &user_object.password);

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

pub fn delete_user_session(session_id: &str) -> bool {
  let mut conn = establish_connection();

  let result = diesel::delete(schema::sessions::table.filter(schema::sessions::id.eq(session_id)))
    .execute(&mut conn);

  match result {
    Ok(count) if count > 0 => true,
    _ => false,
  }
}

pub fn delete_all_user_sessions(user_id: &str) -> bool {
  let mut conn = establish_connection();

  let result =
    diesel::delete(schema::sessions::table.filter(schema::sessions::user_id.eq(user_id)))
      .execute(&mut conn);

  result.is_ok()
}

#[cfg(test)]
mod ci_session {
  use crate::services;

  use super::*;

  #[test]
  fn creates_a_session() {
    let random_name = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", random_name);
    let password = "password".to_string();

    let user = services::CreateUser {
      name: random_name.clone(),
      email: email.clone(),
      password: password.clone(),
      invite: None,
    };

    let created_user = services::create_user(user);

    assert!(created_user.is_some());

    let found_user = services::get_user_by_email(&email);

    assert!(found_user.is_some());

    let credentials = UserCredentials {
      email: email.clone(),
      password: password.clone(),
    };
    let metadata = SessionMetadata {
      ip_address: "SYSTEM".to_string(),
      user_agent: "SYSTEM".to_string(),
    };

    let session = create_user_session(credentials, metadata);

    assert!(session.is_some());

    let found_session = get_user_session_by_id(&session.unwrap().id);

    assert!(found_session.is_some());
  }

  #[test]
  fn deletes_a_session() {
    let random_name = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", random_name);
    let password = "password".to_string();

    let user = services::CreateUser {
      name: random_name.clone(),
      email: email.clone(),
      password: password.clone(),
      invite: None,
    };

    let created_user = services::create_user(user);

    assert!(created_user.is_some());

    let found_user = services::get_user_by_email(&email);

    assert!(found_user.is_some());

    let credentials = UserCredentials {
      email: email.clone(),
      password: password.clone(),
    };
    let metadata = SessionMetadata {
      ip_address: "SYSTEM".to_string(),
      user_agent: "SYSTEM".to_string(),
    };

    let session = create_user_session(credentials, metadata);

    assert!(session.is_some());

    let session_id = session.unwrap().id;
    let found_session = get_user_session_by_id(&session_id);

    assert!(found_session.is_some());

    let deleted = delete_user_session(&session_id);

    assert!(deleted);

    let found_session = get_user_session_by_id(&session_id);

    assert!(found_session.is_none());
  }
}
