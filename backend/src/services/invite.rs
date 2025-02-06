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

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Invite {
  pub id: String,
  pub created_at: i64,
  pub code: String,
  pub used: bool,
}

pub fn get_invite(code: &str) -> Option<Invite> {
  let mut conn = establish_connection();

  let found_invite = schema::invites::table
    .filter(schema::invites::code.eq(&code))
    .first(&mut conn);

  match found_invite {
    Ok(invite) => Some(invite),
    Err(_) => None,
  }
}

pub fn generate_invite(code: Option<&str>) -> Invite {
  let mut conn = establish_connection();

  let code = match code {
    Some(c) => match get_invite(c) {
      Some(_) => generate_invite_code(),
      None => c.to_string(),
    },
    None => generate_invite_code(),
  };

  let new_invite = Invite {
    id: Uuid::new_v4().to_string(),
    created_at: crate::util::unix_time::unix_ms(),
    code,
    used: false,
  };

  diesel::insert_into(schema::invites::table)
    .values(&new_invite)
    .execute(&mut conn)
    .expect("Error saving new invite");

  new_invite
}

#[cfg(test)]
mod ci_invite {
  use super::*;

  #[test]
  fn generates_an_invite() {
    let code = generate_invite(None);
    assert!(!code.code.is_empty());
  }

  #[test]
  fn generates_unique_invites() {
    let code1 = generate_invite(None);
    let code2 = generate_invite(None);
    assert_ne!(code1.code, code2.code);
  }

  #[test]
  fn generates_an_invite_with_code() {
    let value = Uuid::new_v4().to_string();
    let code = generate_invite(Some(&value));
    assert_eq!(code.code, value);
  }

  #[test]
  fn generates_unique_invites_with_code() {
    let value = Uuid::new_v4().to_string();
    let code1 = generate_invite(Some(&value));
    let code2 = generate_invite(Some(&value));
    assert_ne!(code1.code, code2.code);
  }
}
