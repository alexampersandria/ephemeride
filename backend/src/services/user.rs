use crate::{establish_connection, schema, schema::users, util};
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
  pub name: String,
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePassword {
  #[validate(length(min = 7, max = 255))]
  pub password: String,
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
pub struct UserDetails {
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub deleted: bool,
  pub invite: Option<String>,
}

fn user_details(user: User) -> UserDetails {
  UserDetails {
    id: user.id,
    created_at: user.created_at,
    name: user.name,
    email: user.email,
    deleted: user.deleted,
    invite: user.invite,
  }
}

pub fn get_user_id(email: &str) -> Result<String, UserError> {
  let mut conn = establish_connection();

  let result = schema::users::table
    .filter(schema::users::email.eq(email))
    .filter(schema::users::deleted.eq(false))
    .select(schema::users::id)
    .first(&mut conn);

  match result {
    Ok(id) => Ok(id),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn get_user(id: &str) -> Result<UserDetails, UserError> {
  let mut conn = establish_connection();

  let result = schema::users::table
    .filter(schema::users::id.eq(&id))
    .filter(schema::users::deleted.eq(false))
    .first(&mut conn);

  match result {
    Ok(user) => Ok(user_details(user)),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn get_password_hash(id: &str) -> Result<String, UserError> {
  let mut conn: diesel::PgConnection = establish_connection();

  let result = schema::users::table
    .filter(schema::users::id.eq(&id))
    .filter(schema::users::deleted.eq(false))
    .first::<User>(&mut conn);

  match result {
    Ok(user) => Ok(user.password),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn create_user(user: CreateUser) -> Result<UserDetails, UserError> {
  if get_user_id(&user.email).is_ok() {
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
    Ok(_) => Ok(user_details(new_user)),
    Err(_) => Err(UserError::DatabaseError),
  }
}

pub fn delete_user(id: &str) -> Result<bool, UserError> {
  let mut conn = establish_connection();

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

pub fn update_user(id: &str, user: UpdateUser) -> Result<bool, UserError> {
  let mut conn = establish_connection();

  let result = diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set((
      schema::users::name.eq(&user.name),
      schema::users::email.eq(&user.email),
    ))
    .execute(&mut conn);

  match result {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(UserError::NotFound),
  }
}

pub fn update_password(id: &str, password: UpdatePassword) -> Result<bool, UserError> {
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
