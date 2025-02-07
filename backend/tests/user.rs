use ephemeride_backend::services::{auth, user};
use uuid::Uuid;

#[test]
fn create_user_and_get_by_email() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());
  assert_eq!(found_user.unwrap().name, random_name);
}

#[test]
fn create_user_and_delete() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());

  let deleted = match user::delete_user(&found_user.unwrap().id) {
    Ok(deleted) => deleted,
    Err(_) => false,
  };

  assert!(deleted);

  let found_user = user::get_user_id(&email);

  assert!(found_user.is_err());
}

#[test]
fn delete_user_that_does_not_exist() {
  let deleted = match user::delete_user("INVALID_ID") {
    Ok(deleted) => deleted,
    Err(_) => false,
  };

  assert!(!deleted);
}

#[test]
fn updates_user() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());
  let found_user = found_user.unwrap();
  let found_user_name = found_user.name.clone();
  assert_eq!(found_user_name, random_name);

  let new_random_name = Uuid::new_v4().to_string();
  let new_email = format!("{}@example.com", new_random_name);

  let updated_user = user::UpdateUser {
    name: new_random_name.clone(),
    email: new_email.clone(),
  };

  let updated = user::update_user(&found_user.id, updated_user);

  assert!(updated.is_ok());

  let found_user = user::get_user(&found_user.id);

  assert!(found_user.is_ok());
  assert_eq!(found_user.unwrap().name, new_random_name);
}

#[test]
fn creates_a_session() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);
  let password = "password".to_string();

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: password.clone(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let user_id = user::get_user_id(&email);

  assert!(user_id.is_ok());

  let credentials = auth::UserCredentials {
    email: email.clone(),
    password: password.clone(),
  };
  let metadata = auth::SessionMetadata {
    ip_address: "SYSTEM".to_string(),
    user_agent: "SYSTEM".to_string(),
  };

  let session = auth::create_user_session(credentials, metadata);

  assert!(session.is_ok());

  let found_session = auth::get_user_session_by_id(&session.unwrap().id);

  assert!(found_session.is_ok());
}

#[test]
fn deletes_a_session() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);
  let password = "password".to_string();

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: password.clone(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user_id(&email);

  assert!(found_user.is_ok());

  let credentials = auth::UserCredentials {
    email: email.clone(),
    password: password.clone(),
  };
  let metadata = auth::SessionMetadata {
    ip_address: "SYSTEM".to_string(),
    user_agent: "SYSTEM".to_string(),
  };

  let session = auth::create_user_session(credentials, metadata);

  assert!(session.is_ok());

  let session_id = session.unwrap().id;
  let found_session = auth::get_user_session_by_id(&session_id);

  assert!(found_session.is_ok());

  let deleted = auth::delete_user_session(&session_id);

  assert!(deleted.unwrap());

  let found_session = auth::get_user_session_by_id(&session_id);

  assert!(found_session.is_err());
}
