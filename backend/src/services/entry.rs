use std::sync::LazyLock;

use crate::{
  establish_connection,
  schema::{entries, entry_tags},
  services::{
    get_user,
    tag::{get_tag, Tag},
  },
  util::{self, EphemerideError},
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub static DATE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$").unwrap());

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = entries)]
pub struct Entry {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub mood: i32,
  pub entry: Option<String>,
  pub date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateEntry {
  #[validate(regex(path = *DATE_REGEX))]
  pub date: String,
  #[validate(range(min = 1, max = 5))]
  pub mood: i32,
  #[validate(length(min = 0, max = 1000))]
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditEntry {
  #[validate(length(min = 1, max = 255))]
  pub id: String,
  #[validate(regex(path = *DATE_REGEX))]
  pub date: String,
  #[validate(range(min = 1, max = 5))]
  pub mood: i32,
  #[validate(length(min = 0, max = 1000))]
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryWithTags {
  pub id: String,
  pub user_id: String,
  pub date: chrono::NaiveDate,
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

pub fn create_entry(entry: CreateEntry) -> Result<EntryWithTags, EphemerideError> {
  match entry.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let naive_date = match chrono::NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(EphemerideError::BadRequest),
  };

  if get_entry_by_date(naive_date, &entry.user_id).is_ok() {
    return Err(EphemerideError::EntryAlreadyExistsForDate);
  }

  let mut tags: Vec<Tag> = Vec::new();

  for tag_id in &entry.selected_tags {
    let tag = get_tag(tag_id, &entry.user_id);

    if tag.is_ok() {
      tags.push(tag.unwrap());
    }
  }

  let user = get_user(&entry.user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let new_entry = Entry {
    id: Uuid::new_v4().to_string(),
    user_id: entry.user_id.clone(),
    date: naive_date,
    created_at: util::unix_ms(),
    mood: entry.mood,
    entry: entry.entry.clone(),
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
    selected_tags: entry.selected_tags,
  };

  Ok(entry_with_tags)
}

pub fn edit_entry(entry: EditEntry) -> Result<EntryWithTags, EphemerideError> {
  match entry.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let naive_date = match chrono::NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(EphemerideError::BadRequest),
  };

  match get_entry_by_date(naive_date, &entry.user_id) {
    Ok(entry_for_date) => {
      if entry_for_date.id != entry.id {
        return Err(EphemerideError::EntryAlreadyExistsForDate);
      }
    }
    Err(_) => (),
  }

  let mut tags: Vec<Tag> = Vec::new();

  for tag_id in &entry.selected_tags {
    let tag = get_tag(tag_id, &entry.user_id);

    if tag.is_ok() {
      tags.push(tag.unwrap());
    }
  }

  let mut conn = establish_connection();

  let result = diesel::update(
    entries::table
      .filter(entries::id.eq(&entry.id))
      .filter(entries::user_id.eq(&entry.user_id)),
  )
  .set((
    entries::date.eq(&naive_date),
    entries::mood.eq(entry.mood),
    entries::entry.eq(&entry.entry),
  ))
  .execute(&mut conn);

  if result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let delete_result = diesel::delete(
    crate::schema::entry_tags::table.filter(crate::schema::entry_tags::entry_id.eq(&entry.id)),
  )
  .execute(&mut conn);

  if delete_result.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  for tag in &tags {
    let entry_tag = EntryTag {
      id: Uuid::new_v4().to_string(),
      entry_id: entry.id.to_string(),
      tag_id: tag.id.to_string(),
    };

    let tag_result = diesel::insert_into(crate::schema::entry_tags::table)
      .values(&entry_tag)
      .execute(&mut conn);

    if tag_result.is_err() {
      return Err(EphemerideError::DatabaseError);
    }
  }

  get_entry_with_tags(&entry.id, &entry.user_id)
}

pub fn get_entry_by_date(date: chrono::NaiveDate, user_id: &str) -> Result<Entry, EphemerideError> {
  let mut conn = establish_connection();

  let result = entries::table
    .filter(entries::date.eq(date))
    .filter(entries::user_id.eq(user_id))
    .first::<Entry>(&mut conn);

  match result {
    Ok(entry) => Ok(entry),
    Err(_) => Err(EphemerideError::EntryNotFound),
  }
}

pub fn get_entry_with_tags(
  entry_id: &str,
  user_id: &str,
) -> Result<EntryWithTags, EphemerideError> {
  let mut conn = establish_connection();

  let result = entries::table
    .filter(entries::id.eq(entry_id))
    .filter(entries::user_id.eq(user_id))
    .first::<Entry>(&mut conn);

  if result.is_err() {
    return Err(EphemerideError::EntryNotFound);
  }

  let entry = result.unwrap();

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

pub fn delete_entry(entry_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let deleted_entry_tags =
    diesel::delete(entry_tags::table.filter(entry_tags::entry_id.eq(entry_id))).execute(&mut conn);

  if deleted_entry_tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let result = diesel::delete(
    entries::table
      .filter(entries::id.eq(entry_id))
      .filter(entries::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(count) => Ok(count > 0),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_entries_in_range(
  from_date: &str,
  to_date: &str,
  user_id: &str,
) -> Result<Vec<EntryWithTags>, EphemerideError> {
  let from_naive_date = match chrono::NaiveDate::parse_from_str(from_date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(EphemerideError::BadRequest),
  };

  let to_naive_date = match chrono::NaiveDate::parse_from_str(to_date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(EphemerideError::BadRequest),
  };

  let mut conn = establish_connection();

  let results = entries::table
    .filter(entries::date.ge(from_naive_date))
    .filter(entries::date.le(to_naive_date))
    .filter(entries::user_id.eq(user_id))
    .order(entries::date.asc())
    .load::<Entry>(&mut conn);

  if results.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let entries = results.unwrap();
  let mut entries_with_tags: Vec<EntryWithTags> = Vec::new();

  for entry in entries {
    let entry_with_tags = get_entry_with_tags(&entry.id, user_id)?;

    entries_with_tags.push(entry_with_tags);
  }

  Ok(entries_with_tags)
}
