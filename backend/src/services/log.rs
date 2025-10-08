use crate::{
  establish_connection,
  schema::{categories, entries, tags},
  services::get_user,
  util::{self, Color, EphemerideError},
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = categories)]
pub struct Category {
  pub id: String,
  pub name: String,
  pub user_id: String,
  pub created_at: i64,
}

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

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = entries)]
pub struct Entry {
  pub id: String,
  pub user_id: String,
  pub date: String,
  pub created_at: i64,
  pub mood: i32,
  pub entry: Option<String>,
}

pub fn create_category(name: String, user_id: String) -> Result<Category, EphemerideError> {
  let user = get_user(&user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let new_category = Category {
    id: Uuid::new_v4().to_string(),
    name,
    user_id,
    created_at: util::unix_ms(),
  };

  let result = diesel::insert_into(categories::table)
    .values(&new_category)
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(new_category),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn edit_category(
  category_id: &str,
  name: String,
  user_id: &str,
) -> Result<Category, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = diesel::update(
    categories::table
      .filter(categories::id.eq(category_id))
      .filter(categories::user_id.eq(user_id)),
  )
  .set(categories::name.eq(name))
  .execute(&mut conn);

  match result {
    Ok(_) => get_category(category_id, user_id),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_category(category_id: &str, user_id: &str) -> Result<Category, EphemerideError> {
  let mut conn = establish_connection();

  let result = categories::table
    .filter(categories::id.eq(category_id))
    .filter(categories::user_id.eq(user_id))
    .first::<Category>(&mut conn);

  match result {
    Ok(category) => Ok(category),
    Err(_) => Err(EphemerideError::CategoryNotFound),
  }
}

pub fn delete_category(category_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let deleted_tags = delete_all_category_tags(category_id, user_id);

  if deleted_tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let result = diesel::delete(
    categories::table
      .filter(categories::id.eq(category_id))
      .filter(categories::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn create_tag(
  name: String,
  color: String,
  category_id: String,
  user_id: String,
) -> Result<Tag, EphemerideError> {
  let user = get_user(&user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let category = get_category(&category_id, &user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let color_value = Color::from(color.as_str());

  let mut conn = establish_connection();

  let tag = Tag {
    id: Uuid::new_v4().to_string(),
    name,
    color: color_value.to_string(),
    user_id,
    category_id,
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

pub fn edit_tag(
  tag_id: &str,
  name: String,
  color: String,
  user_id: &str,
) -> Result<Tag, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let color_value = Color::from(color.as_str());

  let mut conn = establish_connection();

  let result = diesel::update(
    tags::table
      .filter(tags::id.eq(tag_id))
      .filter(tags::user_id.eq(user_id)),
  )
  .set((tags::name.eq(name), tags::color.eq(color_value.to_string())))
  .execute(&mut conn);

  match result {
    Ok(_) => get_tag(tag_id, user_id),
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

pub fn delete_tag(tag_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = diesel::delete(
    tags::table
      .filter(tags::id.eq(tag_id))
      .filter(tags::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(_) => Ok(true),
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

pub fn get_all_categories(user_id: &str) -> Result<Vec<Category>, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = categories::table
    .filter(categories::user_id.eq(user_id))
    .load::<Category>(&mut conn);

  match result {
    Ok(categories) => Ok(categories),
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

pub fn create_default_data(user_id: String) -> Result<bool, EphemerideError> {
  let default_categories = vec!["Activities", "Tags"];

  let default_tags = vec![
    ("Activities", "Work", "base"),
    ("Activities", "Movie", "base"),
    ("Activities", "Exercise", "base"),
    ("Activities", "Read", "base"),
    ("Activities", "Shopping", "base"),
    ("Activities", "Gaming", "base"),
    ("Tags", "Travel", "base"),
    ("Tags", "Important", "blue"),
    ("Tags", "Sick", "red"),
  ];

  for category_name in default_categories {
    let category = create_category(category_name.to_string(), user_id.clone());

    if category.is_err() {
      return Err(EphemerideError::DatabaseError);
    }

    let category = category.unwrap();

    for (cat_name, tag_name, color) in &default_tags {
      if *cat_name == category_name {
        let tag = create_tag(
          tag_name.to_string(),
          color.to_string(),
          category.id.clone(),
          user_id.clone(),
        );

        if tag.is_err() {
          return Err(EphemerideError::DatabaseError);
        }
      }
    }
  }

  Ok(true)
}
