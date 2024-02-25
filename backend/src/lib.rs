#![forbid(unsafe_code)]

pub mod api;
pub mod schema;
pub mod util;

use diesel::{pg, Connection};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> pg::PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  pg::PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
