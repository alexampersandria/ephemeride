use bcrypt::verify;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::v1::user, util};

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub accessed_at: i64,
  pub ip_address: String,
  pub user_agent: String,
}

pub struct CreateSessionMetadata {
  pub ip_address: String,
  pub user_agent: String,
}

pub fn create_user_session(
  auth_user: user::AuthUser,
  metadata: CreateSessionMetadata,
) -> Option<Session> {
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

  // #TODO: save session to database

  Some(session)
}
