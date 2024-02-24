use crate::establish_connection;
use crate::schema;
use crate::util;

use diesel::prelude::Insertable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use poem::handler;
use poem::http::StatusCode;
use poem::web::Json;
use poem::Response;

use validator::Validate;

use serde::Deserialize;
use serde::Serialize;

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

#[derive(Debug, Deserialize, Serialize, Insertable)]
struct User {
  id: String,
  created_at: i64,
  name: String,
  email: String,
  password: String,
  deleted: bool,
  invite: String,
}

#[handler]
pub fn create_user(Json(user): Json<CreateUser>) -> Response {
  dotenv().ok();

  let mut conn = establish_connection();

  let mut invite_value = "".to_string();

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
          Ok(_) => invite_value = invite.to_string(),
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
  let found_user = schema::users::table
    .filter(schema::users::email.eq(&user.email))
    .select(schema::users::id)
    .first::<String>(&mut conn);

  // check if user already exists, if so return 409 conflict
  match found_user {
    Ok(_) => return Response::builder().status(StatusCode::CONFLICT).body(()),
    Err(_) => (),
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
        .body(())
    }
  }

  return Response::builder().status(StatusCode::CREATED).body(());
}
