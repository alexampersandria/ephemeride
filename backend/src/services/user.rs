use crate::{establish_connection, schema, schema::users, services::auth, util};
use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::delete_all_user_sessions;

#[derive(Debug)]
pub enum UserError {
  EmailAlreadyInUse,
  NotFound,
  InvalidPassword,
  Unauthorized,
  DatabaseError,
}

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
pub struct AuthUser {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 7, max = 255))]
  pub password: String,
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

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct PublicUser {
  pub id: String,
  pub created_at: i64,
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct CurrentUser {
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub deleted: bool,
  pub invite: Option<String>,
}

fn public_user(user: User) -> PublicUser {
  PublicUser {
    id: user.id,
    created_at: user.created_at,
    name: user.name,
  }
}

fn current_user(user: User) -> CurrentUser {
  CurrentUser {
    id: user.id,
    created_at: user.created_at,
    name: user.name,
    email: user.email,
    deleted: user.deleted,
    invite: user.invite,
  }
}

pub fn get_user_by_id(id: &str) -> Result<PublicUser, UserError> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::id.eq(&id))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match found_user {
    Ok(user) => Ok(public_user(user)),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn get_user_by_email(email: &str) -> Result<PublicUser, UserError> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::email.eq(&email))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match found_user {
    Ok(user) => Ok(public_user(user)),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn get_current_user(session_id: &str) -> Result<CurrentUser, UserError> {
  let mut conn = establish_connection();

  let found_session = auth::get_user_session_by_id(session_id).map_err(|_| UserError::NotFound)?;

  let found_user = schema::users::table
    .filter(schema::users::id.eq(&found_session.user_id))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match found_user {
    Ok(user) => Ok(current_user(user)),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn get_password_hash(id: &str) -> Result<String, UserError> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::id.eq(&id))
    .filter(schema::users::deleted.eq(false))
    .first::<User>(&mut conn);

  match found_user {
    Ok(user) => Ok(user.password),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn create_user(user: CreateUser) -> Result<CurrentUser, UserError> {
  let found_user = get_user_by_email(&user.email);

  if found_user.is_ok() {
    return Err(UserError::EmailAlreadyInUse);
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
    Ok(_) => Ok(current_user(new_user)),
    Err(_) => Err(UserError::DatabaseError),
  }
}

pub fn delete_user(id: &str) -> Result<bool, UserError> {
  let mut conn = establish_connection();

  let found_user = get_user_by_id(id);
  match found_user {
    Ok(_) => (),
    Err(_) => return Err(UserError::NotFound),
  }

  let deleted_sessions = delete_all_user_sessions(id);
  match deleted_sessions {
    Ok(_) => (),
    Err(_) => return Err(UserError::DatabaseError),
  }

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set(schema::users::deleted.eq(true))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(UserError::DatabaseError),
  }
}

pub fn update_user(id: &str, user: UpdateUser) -> Result<PublicUser, UserError> {
  let found_user = get_user_by_id(id);
  if found_user.is_err() {
    return Err(UserError::NotFound);
  }

  let mut conn = establish_connection();

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set((
      schema::users::name.eq(&user.name),
      schema::users::email.eq(&user.email),
    ))
    .execute(&mut conn);

  match result {
    Ok(_) => get_user_by_id(id),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn update_password(id: &str, password: UpdatePassword) -> Result<bool, UserError> {
  let found_user = get_user_by_id(id);
  match found_user {
    Ok(_) => (),
    Err(_) => return Err(UserError::NotFound),
  }

  let mut conn = establish_connection();

  let password_hash = bcrypt::hash(password.password, bcrypt::DEFAULT_COST).unwrap();

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set(schema::users::password.eq(&password_hash))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(UserError::DatabaseError),
  }
}

#[cfg(test)]
mod tests {
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

    assert!(created_user.is_ok());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_ok());
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

    assert!(created_user.is_ok());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_ok());

    let deleted = delete_user(&found_user.unwrap().id);

    assert!(deleted.is_ok());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_err());
  }

  #[test]
  fn delete_user_that_does_not_exist() {
    let deleted = delete_user("INVALID_ID");

    assert!(deleted.is_err());
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

    assert!(created_user.is_ok());

    let found_user = get_user_by_email(&email);

    assert!(found_user.is_ok());
    let found_user: PublicUser = found_user.unwrap();
    let found_user_name = found_user.name.clone();
    assert_eq!(found_user_name, random_name);

    let new_random_name = Uuid::new_v4().to_string();
    let new_email = format!("{}@example.com", new_random_name);

    let updated_user = UpdateUser {
      name: new_random_name.clone(),
      email: new_email.clone(),
    };

    let updated = update_user(&found_user.id, updated_user);

    assert!(updated.is_ok());

    let found_user = get_user_by_email(&new_email);

    assert!(found_user.is_ok());
    assert_eq!(found_user.unwrap().name, new_random_name);
  }
}
