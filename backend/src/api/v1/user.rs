use crate::{
  establish_connection,
  schema::{self, users},
  services::{session, user, UserCredentials},
  util,
};

use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, QueryDsl, RunQueryDsl,
};

use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

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
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub password: String,
  pub deleted: bool,
  pub invite: Option<String>,
}

// #TODO: use services instead of direct calls

#[handler]
pub fn create_user(Json(user): Json<CreateUser>, request: &Request) -> Response {
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
  let found_user = user::get_user_by_email(&user.email);

  // check if user already exists, if so return 409 conflict
  match found_user {
    Some(_) => return Response::builder().status(StatusCode::CONFLICT).body(()),
    None => (),
  }

  // create user object
  let new_user = User {
    id: Uuid::new_v4().to_string(),
    created_at: util::unix_time::unix_ms(),
    name: String::from(&user.name),
    email: String::from(&user.email),
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

  let session = session::create_user_session(
    UserCredentials {
      email: String::from(&user.email),
      password: String::from(&user.password),
    },
    session::SessionMetadata {
      ip_address: request.remote_addr().to_string(),
      user_agent: request
        .header("user-agent")
        .unwrap_or("unknown")
        .to_string(),
    },
  );

  match session {
    Some(session) => {
      Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Authorization", &session.id)
        .body(serde_json::to_string(&session).unwrap())
    }
    None => {
      Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(())
    }
  }
}

#[handler]
pub fn auth_user(Json(user): Json<AuthUser>, request: &Request) -> Response {
  dotenv().ok();

  let found_user = user::get_user_by_email(&user.email);

  match found_user {
    Some(user_object) => {
      let valid_password = bcrypt::verify(&user.password, &user_object.password);

      match valid_password {
        Ok(_) => (),
        Err(_) => {
          return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(());
        }
      }

      let session = session::create_user_session(
        UserCredentials {
          email: String::from(&user.email),
          password: String::from(&user.password),
        },
        session::SessionMetadata {
          ip_address: request.remote_addr().to_string(),
          user_agent: request
            .header("user-agent")
            .unwrap_or("unknown")
            .to_string(),
        },
      );

      match session {
        Some(session) => {
          Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("Authorization", &session.id)
            .body(serde_json::to_string(&session).unwrap())
        }
        None => {
          Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(())
        }
      }
    }
    None => {
      Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(())
    }
  }
}
