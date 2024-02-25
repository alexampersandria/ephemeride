use crate::{establish_connection, schema, util};

use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, QueryDsl, RunQueryDsl,
};

use poem::{handler, http::StatusCode, web::Json, Response};

use validator::Validate;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::schema::users;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUser {
  #[validate(length(min = 1, max = 255))]
  name: String,
  #[validate(email)]
  email: String,
  #[validate(length(min = 7, max = 255))]
  password: String,
  invite: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AuthUser {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 7, max = 255))]
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct User {
  id: String,
  created_at: i64,
  name: String,
  email: String,
  password: String,
  deleted: bool,
  invite: Option<String>,
}

pub fn get_user_by_email(email: &str) -> Option<User> {
  let mut conn = establish_connection();

  let found_user = schema::users::table
    .filter(schema::users::email.eq(&email))
    .first(&mut conn);

  match found_user {
    Ok(user) => Some(user),
    Err(_) => None,
  }
}

#[handler]
pub fn create_user(Json(user): Json<CreateUser>) -> Response {
  dotenv().ok();

  let mut conn = establish_connection();

  let mut invite_value = None;

  // check if INVITE_REQUIRED .env variable is set to true, if so check if invite is valid
  if env::var("INVITE_REQUIRED").unwrap_or("false".to_string()) == "true" {
    match &user.invite {
      Some(invite) => {
        // search for invite
        let found_invite = schema::invites::table
          .filter(schema::invites::code.eq(&invite))
          .select(schema::invites::id)
          .first::<String>(&mut conn);

        // check if invite exists, if not return 401 unauthorized
        match found_invite {
          Ok(_) => invite_value = Some(invite.to_string()),
          Err(_) => {
            return Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .body(())
          }
        }
      }
      // if invite is not provided but required in .env return 401 unauthorized
      None => {
        return Response::builder()
          .status(StatusCode::UNAUTHORIZED)
          .body(())
      }
    }
  }

  // validate user object, if invalid return 400 bad request
  match user.validate() {
    Ok(_) => (),
    Err(_) => return Response::builder().status(StatusCode::BAD_REQUEST).body(()),
  }

  // search for user with the same email
  let found_user = get_user_by_email(&user.email);

  // check if user already exists, if so return 409 conflict
  match found_user {
    Some(_) => return Response::builder().status(StatusCode::CONFLICT).body(()),
    None => (),
  }

  // create user object
  let new_user = User {
    id: Uuid::new_v4().to_string(),
    created_at: util::unix_time::unix_ms(),
    name: user.name,
    email: user.email,
    password: bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap(),
    deleted: false,
    invite: invite_value,
  };

  // insert user into database
  let result = diesel::insert_into(schema::users::table)
    .values(&new_user)
    .execute(&mut conn);

  // check if user was inserted, if not return 500 internal server error
  match result {
    Ok(_) => (),
    Err(_) => {
      return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(());
    }
  }

  // #TODO: create session and return token, for now just return 201 created
  return Response::builder()
    .status(StatusCode::CREATED)
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&new_user).unwrap());
}
