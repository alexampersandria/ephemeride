use crate::{
  establish_connection,
  schema::{entry_tags, tags},
  services::{get_user, category::get_category},
  util::{self, Color, EphemerideError},
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = tags)]
pub struct Tag {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub name: String,
  pub color: String,
  pub category_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTag {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 16))]
  pub color: String,
  #[validate(length(min = 1, max = 255))]
  pub category_id: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditTag {
  #[validate(length(min = 1, max = 255))]
  pub id: String,
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 16))]
  pub color: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

pub fn create_tag(tag: CreateTag) -> Result<Tag, EphemerideError> {
  match tag.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let user = get_user(&tag.user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let category = get_category(&tag.category_id, &tag.user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let color_value = Color::from(tag.color.as_str());

  let mut conn = establish_connection();

  let tag = Tag {
    id: Uuid::new_v4().to_string(),
    name: tag.name,
    color: color_value.to_string(),
    user_id: tag.user_id,
    category_id: tag.category_id,
    created_at: util::unix_ms(),
  };

  let tag_result = diesel::insert_into(tags::table)
    .values(&tag)
    .execute(&mut conn);

  match tag_result {
    Ok(_) => Ok(tag),
    _ => Err(EphemerideError::DatabaseError),
  }
}

pub fn edit_tag(tag: EditTag) -> Result<Tag, EphemerideError> {
  match tag.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let user = get_user(&tag.user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let color_value = Color::from(tag.color.as_str());

  let mut conn = establish_connection();

  let result = diesel::update(
    tags::table
      .filter(tags::id.eq(&tag.id))
      .filter(tags::user_id.eq(&tag.user_id)),
  )
  .set((
    tags::name.eq(&tag.name),
    tags::color.eq(color_value.to_string()),
  ))
  .execute(&mut conn);

  match result {
    Ok(_) => get_tag(&tag.id, &tag.user_id),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_tag(tag_id: &str, user_id: &str) -> Result<Tag, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = tags::table
    .filter(tags::id.eq(tag_id))
    .filter(tags::user_id.eq(user_id))
    .first::<Tag>(&mut conn);

  match result {
    Ok(tag) => Ok(tag),
    Err(_) => Err(EphemerideError::TagNotFound),
  }
}

pub fn get_tags(tag_ids: Vec<&str>, user_id: &str) -> Result<Vec<Tag>, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = tags::table
    .filter(tags::id.eq_any(tag_ids))
    .filter(tags::user_id.eq(user_id))
    .load::<Tag>(&mut conn);

  match result {
    Ok(tags) => Ok(tags),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn delete_tag(tag_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let deleted_entry_tags =
    diesel::delete(entry_tags::table.filter(entry_tags::tag_id.eq(tag_id))).execute(&mut conn);

  if deleted_entry_tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let result = diesel::delete(
    tags::table
      .filter(tags::id.eq(tag_id))
      .filter(tags::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(count) => Ok(count > 0),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn delete_all_category_tags(category_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let mut conn = establish_connection();

  let result = diesel::delete(
    tags::table
      .filter(tags::category_id.eq(category_id))
      .filter(tags::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_category_tags(category_id: &str, user_id: &str) -> Result<Vec<Tag>, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let mut conn = establish_connection();

  let result = tags::table
    .filter(tags::category_id.eq(category_id))
    .filter(tags::user_id.eq(user_id))
    .load::<Tag>(&mut conn);

  match result {
    Ok(tags) => Ok(tags),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}
