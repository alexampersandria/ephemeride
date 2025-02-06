use crate::{establish_connection, schema, schema::users, util};
use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::delete_all_user_sessions;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUser {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 7, max = 255))]
  pub password: String,
  pub invite: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUser {
  #[validate(length(min = 1, max = 255))]
  name: String,
  #[validate(email)]
  email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePassword {
  #[validate(length(min = 7, max = 255))]
  password: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct User {
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub password: String,
  pub deleted: bool,
  pub invite: Option<String>,
}

pub fn get_user_by_id(id: &str) -> Option<User> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::id.eq(&id))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match found_user {
    Ok(user) => Some(user),
    Err(_) => None,
  }
}

pub fn get_user_by_email(email: &str) -> Option<User> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::email.eq(&email))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match found_user {
    Ok(user) => Some(user),
    Err(_) => None,
  }
}

pub fn create_user(user: CreateUser) -> Option<User> {
  let found_user = get_user_by_email(&user.email);
  match found_user {
    Some(_) => return None,
    None => (),
  }

  let mut conn = establish_connection();

  let password_hash = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();

  let new_user = User {
    id: Uuid::new_v4().to_string(),
    created_at: util::unix_time::unix_ms(),
    name: user.name,
    email: user.email,
    password: password_hash,
    deleted: false,
    invite: user.invite,
  };

  let result = diesel::insert_into(schema::users::table)
    .values(&new_user)
    .execute(&mut conn);

  match result {
    Ok(_) => Some(new_user),
    Err(_) => None,
  }
}

pub fn delete_user(id: &str) -> bool {
  let mut conn = establish_connection();

  let deleted_sessions = delete_all_user_sessions(id);
  if !deleted_sessions {
    return false;
  }

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set(schema::users::deleted.eq(true))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) if rows_affected > 0 => true,
    _ => false,
  }
}

pub fn update_user(id: &str, user: UpdateUser) -> bool {
  let found_user = get_user_by_id(id);
  match found_user {
    Some(_) => (),
    None => return false,
  }

  let mut conn = establish_connection();

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set((
      schema::users::name.eq(&user.name),
      schema::users::email.eq(&user.email),
    ))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) if rows_affected > 0 => true,
    _ => false,
  }
}

pub fn update_password(id: &str, password: UpdatePassword) -> bool {
  let found_user = get_user_by_id(id);
  match found_user {
    Some(_) => (),
    None => return false,
  }

  let mut conn = establish_connection();

  let password_hash = bcrypt::hash(&password.password, bcrypt::DEFAULT_COST).unwrap();

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set(schema::users::password.eq(&password_hash))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) if rows_affected > 0 => true,
    _ => false,
  }
}

#[cfg(test)]
mod ci_user {
  use super::*;

  #[test]
  fn create_user_and_get_by_email() {
    let random_name = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", random_name);

    let user = CreateUser {
      name: random_name.clone(),
      email: email.clone(),
      password: "password".to_string(),
      invite: None,
    };

    let created_user = create_user(user);

    assert!(created_user.is_some());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().name, random_name);
  }

  #[test]
  fn create_user_and_delete() {
    let random_name = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", random_name);

    let user = CreateUser {
      name: random_name.clone(),
      email: email.clone(),
      password: "password".to_string(),
      invite: None,
    };

    let created_user = create_user(user);

    assert!(created_user.is_some());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_some());

    let deleted = delete_user(&found_user.unwrap().id);

    assert!(deleted);

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_none());
  }

  #[test]
  fn delete_user_that_does_not_exist() {
    let deleted = delete_user("INVALID_ID");

    assert!(!deleted);
  }

  #[test]
  fn updates_user() {
    let random_name = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", random_name);

    let user = CreateUser {
      name: random_name.clone(),
      email: email.clone(),
      password: "password".to_string(),
      invite: None,
    };

    let created_user = create_user(user);

    assert!(created_user.is_some());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    let found_user_name = found_user.name.clone();
    assert_eq!(found_user_name, random_name);

    let new_random_name = Uuid::new_v4().to_string();
    let new_email = format!("{}@example.com", new_random_name);

    let updated_user = UpdateUser {
      name: new_random_name.clone(),
      email: new_email.clone(),
    };

    let updated = update_user(&found_user.id, updated_user);

    assert!(updated);

    let found_user = get_user_by_email(&new_email);

    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().name, new_random_name);
  }
}
