use crate::{
  errors::EphemerideError,
  establish_connection,
  schema::{self, invites},
  util::generate_invite_code,
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Invite {
  pub id: String,
  pub created_at: i64,
  pub code: String,
  pub used: bool,
}

pub fn get_invite(code: &str) -> Result<Invite, EphemerideError> {
  let mut conn = establish_connection();

  let result = schema::invites::table
    .filter(schema::invites::code.eq(&code))
    .first(&mut conn);

  match result {
    Ok(invite) => Ok(invite),
    Err(_) => Err(EphemerideError::InviteNotFound),
  }
}

pub fn use_invite(code: &str) -> Result<Invite, EphemerideError> {
  let mut conn = establish_connection();

  let invite = match get_invite(code) {
    Ok(invite) => invite,
    Err(e) => return Err(e),
  };

  if invite.used {
    return Err(EphemerideError::InviteUsed);
  }

  let result = diesel::update(schema::invites::table.filter(schema::invites::code.eq(&code)))
    .set(schema::invites::used.eq(true))
    .get_result(&mut conn);

  match result {
    Ok(invite) => Ok(invite),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn generate_invite(code: Option<&str>) -> Result<Invite, EphemerideError> {
  let mut conn = establish_connection();

  let code = match code {
    Some(c) => match get_invite(c) {
      Ok(_) => generate_invite_code(),
      Err(_) => c.to_string(),
    },
    None => generate_invite_code(),
  };

  let new_invite = Invite {
    id: Uuid::new_v4().to_string(),
    created_at: crate::util::unix_time::unix_ms(),
    code,
    used: false,
  };

  let result = diesel::insert_into(schema::invites::table)
    .values(&new_invite)
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(new_invite),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}
