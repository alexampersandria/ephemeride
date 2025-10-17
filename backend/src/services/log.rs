use crate::{
  establish_connection,
  schema::{categories, entries, entry_tags, tags},
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryWithTags {
  pub id: String,
  pub name: String,
  pub user_id: String,
  pub created_at: i64,
  pub tags: Vec<Tag>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryWithTags {
  pub id: String,
  pub user_id: String,
  pub date: String,
  pub created_at: i64,
  pub mood: i32,
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = entry_tags)]
pub struct EntryTag {
  pub id: String,
  pub entry_id: String,
  pub tag_id: String,
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

pub fn get_category_with_tags(
  category_id: &str,
  user_id: &str,
) -> Result<CategoryWithTags, EphemerideError> {
  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let tags = get_category_tags(category_id, user_id);

  if tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let category = category.unwrap();
  let tags = tags.unwrap();

  let category_with_tags = CategoryWithTags {
    id: category.id,
    name: category.name,
    user_id: category.user_id,
    created_at: category.created_at,
    tags,
  };

  Ok(category_with_tags)
}

pub fn get_user_categories_with_tags(
  user_id: &str,
) -> Result<Vec<CategoryWithTags>, EphemerideError> {
  let categories = get_all_categories(user_id);

  if categories.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let categories = categories.unwrap();
  let mut categories_with_tags: Vec<CategoryWithTags> = Vec::new();

  for category in categories {
    let tags = get_category_tags(&category.id, user_id);

    if tags.is_err() {
      return Err(EphemerideError::DatabaseError);
    }

    let category_with_tags = CategoryWithTags {
      id: category.id,
      name: category.name,
      user_id: category.user_id,
      created_at: category.created_at,
      tags: tags.unwrap(),
    };

    categories_with_tags.push(category_with_tags);
  }

  Ok(categories_with_tags)
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

pub fn create_entry(
  date: String,
  mood: i32,
  entry_content: Option<String>,
  selected_tags: Vec<String>,
  user_id: String,
) -> Result<EntryWithTags, EphemerideError> {
  let mut tags: Vec<Tag> = Vec::new();

  for tag_id in &selected_tags {
    let tag = get_tag(tag_id, &user_id);

    if tag.is_ok() {
      tags.push(tag.unwrap());
    }
  }

  let user = get_user(&user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let new_entry = Entry {
    id: Uuid::new_v4().to_string(),
    user_id,
    date,
    created_at: util::unix_ms(),
    mood,
    entry: entry_content,
  };

  let result = diesel::insert_into(entries::table)
    .values(&new_entry)
    .execute(&mut conn);

  if result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  for tag in &tags {
    let entry_tag = EntryTag {
      id: Uuid::new_v4().to_string(),
      entry_id: new_entry.id.clone(),
      tag_id: tag.id.clone(),
    };

    let tag_result = diesel::insert_into(crate::schema::entry_tags::table)
      .values(&entry_tag)
      .execute(&mut conn);

    if tag_result.is_err() {
      return Err(EphemerideError::DatabaseError);
    }
  }

  let entry_with_tags = EntryWithTags {
    id: new_entry.id,
    user_id: new_entry.user_id,
    date: new_entry.date,
    created_at: new_entry.created_at,
    mood: new_entry.mood,
    entry: new_entry.entry,
    selected_tags: selected_tags,
  };

  Ok(entry_with_tags)
}

pub fn edit_entry(
  entry_id: &str,
  date: String,
  mood: i32,
  entry_content: Option<String>,
  selected_tags: Vec<String>,
  user_id: &str,
) -> Result<EntryWithTags, EphemerideError> {
  let mut conn = establish_connection();

  let result = diesel::update(
    entries::table
      .filter(entries::id.eq(entry_id))
      .filter(entries::user_id.eq(user_id)),
  )
  .set((
    entries::date.eq(date),
    entries::mood.eq(mood),
    entries::entry.eq(entry_content),
  ))
  .execute(&mut conn);

  if result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let delete_result = diesel::delete(
    crate::schema::entry_tags::table.filter(crate::schema::entry_tags::entry_id.eq(entry_id)),
  )
  .execute(&mut conn);

  if delete_result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  for tag_id in &selected_tags {
    let entry_tag = EntryTag {
      id: Uuid::new_v4().to_string(),
      entry_id: entry_id.to_string(),
      tag_id: tag_id.to_string(),
    };

    let tag_result = diesel::insert_into(crate::schema::entry_tags::table)
      .values(&entry_tag)
      .execute(&mut conn);

    if tag_result.is_err() {
      return Err(EphemerideError::DatabaseError);
    }
  }

  get_entry_with_tags(entry_id, user_id)
}

pub fn get_entry_with_tags(
  entry_id: &str,
  user_id: &str,
) -> Result<EntryWithTags, EphemerideError> {
  let mut conn = establish_connection();

  let entry_result = entries::table
    .filter(entries::id.eq(entry_id))
    .filter(entries::user_id.eq(user_id))
    .first::<Entry>(&mut conn);

  if entry_result.is_err() {
    return Err(EphemerideError::EntryNotFound);
  }

  let entry = entry_result.unwrap();

  let entry_tags_result = crate::schema::entry_tags::table
    .filter(crate::schema::entry_tags::entry_id.eq(entry_id))
    .load::<EntryTag>(&mut conn);

  if entry_tags_result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let entry_tags = entry_tags_result.unwrap();
  let tag_ids: Vec<String> = entry_tags.into_iter().map(|et| et.tag_id).collect();

  let entry_with_tags = EntryWithTags {
    id: entry.id,
    user_id: entry.user_id,
    date: entry.date,
    created_at: entry.created_at,
    mood: entry.mood,
    entry: entry.entry,
    selected_tags: tag_ids,
  };

  Ok(entry_with_tags)
}
