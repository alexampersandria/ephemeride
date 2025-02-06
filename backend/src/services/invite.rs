use crate::{
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

#[derive(Debug)]
pub enum InviteError {
  NotFound,
  Used,
  DatabaseError,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Invite {
  pub id: String,
  pub created_at: i64,
  pub code: String,
  pub used: bool,
}

pub fn get_invite(code: &str) -> Result<Invite, InviteError> {
  let mut conn = establish_connection();

  let found_invite = schema::invites::table
    .filter(schema::invites::code.eq(&code))
    .first(&mut conn);

  match found_invite {
    Ok(invite) => Ok(invite),
    Err(_) => Err(InviteError::NotFound),
  }
}

pub fn use_invite(code: &str) -> Result<Invite, InviteError> {
  let mut conn = establish_connection();

  let found_invite = get_invite(code);

  let invite = match found_invite {
    Ok(invite) => invite,
    Err(e) => return Err(e),
  };

  if invite.used {
    return Err(InviteError::Used);
  }

  let result = diesel::update(schema::invites::table.filter(schema::invites::code.eq(&code)))
    .set(schema::invites::used.eq(true))
    .get_result(&mut conn);

  match result {
    Ok(invite) => Ok(invite),
    Err(_) => Err(InviteError::DatabaseError),
  }
}

pub fn generate_invite(code: Option<&str>) -> Result<Invite, InviteError> {
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
    Err(_) => Err(InviteError::DatabaseError),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn generates_an_invite() {
    let code = generate_invite(None);
    assert!(code.is_ok());
  }

  #[test]
  fn generates_unique_invites() {
    let code1 = generate_invite(None).unwrap();
    let code2 = generate_invite(None).unwrap();
    assert_ne!(code1.code, code2.code);
  }

  #[test]
  fn generates_an_invite_with_code() {
    let value = Uuid::new_v4().to_string();
    let code = generate_invite(Some(&value)).unwrap();
    assert_eq!(code.code, value);
  }

  #[test]
  fn generates_unique_invites_with_code() {
    let value = Uuid::new_v4().to_string();
    let code1 = generate_invite(Some(&value)).unwrap();
    let code2 = generate_invite(Some(&value)).unwrap();
    assert_ne!(code1.code, code2.code);
  }
}
