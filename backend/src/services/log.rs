// moved category, entry, and tag to their own modules
// exporting from log for backwards compatibility
// but usage should migrate to the new modules
// #TODO: move usage, see above :)
pub use super::category::*;
pub use super::entry::*;
pub use super::tag::*;

use crate::{
  establish_connection,
  schema::{categories, entries, tags},
  services::get_user,
  util::EphemerideError,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

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
    let category = create_category(CreateCategory {
      name: category_name.to_string(),
      user_id: user_id.clone(),
    });

    if category.is_err() {
      return Err(EphemerideError::DatabaseError);
    }

    let category = category.unwrap();

    for (cat_name, tag_name, color) in &default_tags {
      if *cat_name == category_name {
        let tag = create_tag(CreateTag {
          name: tag_name.to_string(),
          color: color.to_string(),
          category_id: category.id.clone(),
          user_id: user_id.clone(),
        });

        if tag.is_err() {
          return Err(EphemerideError::DatabaseError);
        }
      }
    }
  }

  Ok(true)
}

pub fn delete_all_user_data(user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let delete_entries =
    diesel::delete(entries::table.filter(entries::user_id.eq(user_id))).execute(&mut conn);

  if delete_entries.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let delete_tags =
    diesel::delete(tags::table.filter(tags::user_id.eq(user_id))).execute(&mut conn);

  if delete_tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let delete_categories =
    diesel::delete(categories::table.filter(categories::user_id.eq(user_id))).execute(&mut conn);

  if delete_categories.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  Ok(true)
}
