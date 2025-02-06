use diesel::{
  deserialize::Queryable, prelude::Insertable, query_dsl::methods::FilterDsl, ExpressionMethods,
  RunQueryDsl,
};
use poem::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{establish_connection, schema, schema::sessions, services::user, util};

#[derive(Debug)]
pub enum SessionError {
  NotFound,
  InvalidPassword,
  DatabaseError,
}

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

pub fn session_metadata(request: &Request) -> SessionMetadata {
  SessionMetadata {
    ip_address: request.remote_addr().to_string(),
    user_agent: request
      .header("user-agent")
      .unwrap_or("unknown")
      .to_string(),
  }
}

fn token_from_header(request: &Request) -> Option<String> {
  let token = request.header("Authorization");
  token.map(|token| token.replace("Bearer ", ""))
}

pub fn authorize_request(request: &Request) -> Result<Session, SessionError> {
  match token_from_header(request) {
    Some(token) => get_user_session_by_id(&token),
    None => Err(SessionError::NotFound),
  }
}

pub fn create_user_session(
  user_credentials: UserCredentials,
  metadata: SessionMetadata,
) -> Result<Session, SessionError> {
  let mut conn = establish_connection();

  let user_id = match user::get_user_id(&user_credentials.email) {
    Ok(id) => id,
    Err(_) => return Err(SessionError::NotFound),
  };

  let password_hash = match user::get_password_hash(&user_id) {
    Ok(hash) => hash,
    Err(_) => return Err(SessionError::DatabaseError),
  };

  let valid_password = bcrypt::verify(&user_credentials.password, &password_hash);

  match valid_password {
    Ok(_) => (),
    Err(_) => return Err(SessionError::InvalidPassword),
  }

  let session = Session {
    id: Uuid::new_v4().to_string(),
    user_id,
    created_at: util::unix_time::unix_ms(),
    accessed_at: util::unix_time::unix_ms(),
    ip_address: metadata.ip_address,
    user_agent: metadata.user_agent,
  };

  let result = diesel::insert_into(schema::sessions::table)
    .values(&session)
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(session),
    Err(_) => Err(SessionError::DatabaseError),
  }
}

pub fn update_accessed_at(session_id: &str) -> Result<bool, SessionError> {
  let mut conn = establish_connection();

  let result = diesel::update(schema::sessions::table.filter(schema::sessions::id.eq(session_id)))
    .set(schema::sessions::accessed_at.eq(util::unix_time::unix_ms()))
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
    Err(_) => Err(SessionError::DatabaseError),
  }
}

pub fn get_user_session_by_id(session_id: &str) -> Result<Session, SessionError> {
  let mut conn = establish_connection();

  let result = schema::sessions::table
    .filter(schema::sessions::id.eq(session_id))
    .first::<Session>(&mut conn);

  match update_accessed_at(session_id) {
    Ok(_) => (),
    Err(_) => return Err(SessionError::DatabaseError),
  }

  match result {
    Ok(session) => Ok(session),
    Err(_) => Err(SessionError::NotFound),
  }
}

pub fn get_all_user_sessions(session_id: &str) -> Result<Vec<Session>, SessionError> {
  let mut conn = establish_connection();

  let current_session = match get_user_session_by_id(session_id) {
    Ok(session) => session,
    Err(_) => return Err(SessionError::NotFound),
  };

  let result = schema::sessions::table
    .filter(schema::sessions::user_id.eq(current_session.user_id))
    .load::<Session>(&mut conn);

  match result {
    Ok(sessions) => Ok(sessions),
    Err(_) => Err(SessionError::DatabaseError),
  }
}

pub fn delete_user_session(session_id: &str) -> Result<bool, SessionError> {
  let mut conn = establish_connection();

  let result = diesel::delete(schema::sessions::table.filter(schema::sessions::id.eq(session_id)))
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
    Err(_) => Err(SessionError::DatabaseError),
  }
}

pub fn delete_all_user_sessions(user_id: &str) -> Result<bool, SessionError> {
  let mut conn = establish_connection();

  let result =
    diesel::delete(schema::sessions::table.filter(schema::sessions::user_id.eq(user_id)))
      .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
    Err(_) => Err(SessionError::DatabaseError),
  }
}

#[cfg(test)]
mod tests {
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

    assert!(created_user.is_ok());

    let user_id = services::get_user_id(&email);

    assert!(user_id.is_ok());

    let credentials = UserCredentials {
      email: email.clone(),
      password: password.clone(),
    };
    let metadata = SessionMetadata {
      ip_address: "SYSTEM".to_string(),
      user_agent: "SYSTEM".to_string(),
    };

    let session = create_user_session(credentials, metadata);

    assert!(session.is_ok());

    let found_session = get_user_session_by_id(&session.unwrap().id);

    assert!(found_session.is_ok());
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

    assert!(created_user.is_ok());

    let found_user = services::get_user_id(&email);

    assert!(found_user.is_ok());

    let credentials = UserCredentials {
      email: email.clone(),
      password: password.clone(),
    };
    let metadata = SessionMetadata {
      ip_address: "SYSTEM".to_string(),
      user_agent: "SYSTEM".to_string(),
    };

    let session = create_user_session(credentials, metadata);

    assert!(session.is_ok());

    let session_id = session.unwrap().id;
    let found_session = get_user_session_by_id(&session_id);

    assert!(found_session.is_ok());

    let deleted = delete_user_session(&session_id);

    assert!(deleted.unwrap());

    let found_session = get_user_session_by_id(&session_id);

    assert!(found_session.is_err());
  }
}
