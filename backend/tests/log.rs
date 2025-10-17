use ephemeride_backend::services::{log, user};
use uuid::Uuid;

fn create_test_user() -> user::UserDetails {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{}@example.com", random_name);

  let user_data = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  user::create_user(user_data).expect("Failed to create test user")
}

#[test]
fn test_create_category() {
  let user = create_test_user();

  let category = log::create_category("Test Category".to_string(), user.id.clone());

  assert!(category.is_ok());
  let category = category.unwrap();
  assert_eq!(category.name, "Test Category");
  assert_eq!(category.user_id, user.id);
}

#[test]
fn test_edit_category() {
  let user = create_test_user();

  let category = log::create_category("Original Name".to_string(), user.id.clone()).unwrap();

  let edited = log::edit_category(&category.id, "Updated Name".to_string(), &user.id);

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(edited.name, "Updated Name");
  assert_eq!(edited.id, category.id);
}

#[test]
fn test_get_category() {
  let user = create_test_user();

  let category = log::create_category("Get Test".to_string(), user.id.clone()).unwrap();

  let found = log::get_category(&category.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, category.id);
  assert_eq!(found.name, "Get Test");
}

#[test]
fn test_get_all_categories() {
  let user = create_test_user();

  log::create_category("Category 1".to_string(), user.id.clone()).unwrap();
  log::create_category("Category 2".to_string(), user.id.clone()).unwrap();

  let categories = log::get_all_categories(&user.id);

  assert!(categories.is_ok());
  let categories = categories.unwrap();
  assert!(categories.len() >= 2);
}

#[test]
fn test_get_category_with_tags() {
  let user = create_test_user();

  let category = log::create_category("Category with Tags".to_string(), user.id.clone()).unwrap();
  log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let category_with_tags = log::get_category_with_tags(&category.id, &user.id);

  assert!(category_with_tags.is_ok());
  let category_with_tags = category_with_tags.unwrap();
  assert_eq!(category_with_tags.id, category.id);
  assert_eq!(category_with_tags.tags.len(), 2);
}

#[test]
fn test_get_user_categories_with_tags() {
  let user = create_test_user();

  let cat1 = log::create_category("Category 1".to_string(), user.id.clone()).unwrap();
  let cat2 = log::create_category("Category 2".to_string(), user.id.clone()).unwrap();

  log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    cat1.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    cat2.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let categories_with_tags = log::get_user_categories_with_tags(&user.id);

  assert!(categories_with_tags.is_ok());
  let categories_with_tags = categories_with_tags.unwrap();
  assert!(categories_with_tags.len() >= 2);

  for category in &categories_with_tags {
    if category.id == cat1.id || category.id == cat2.id {
      assert!(category.tags.len() > 0);
    }
  }
}

#[test]
fn test_delete_category() {
  let user = create_test_user();

  let category = log::create_category("To Delete".to_string(), user.id.clone()).unwrap();
  log::create_tag(
    "Tag".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let deleted = log::delete_category(&category.id, &user.id);

  assert!(deleted.is_ok());
  assert_eq!(deleted.unwrap(), true);

  let found = log::get_category(&category.id, &user.id);
  assert!(found.is_err());
}

#[test]
fn test_create_tag() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();

  let tag = log::create_tag(
    "Test Tag".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  );

  assert!(tag.is_ok());
  let tag = tag.unwrap();
  assert_eq!(tag.name, "Test Tag");
  assert_eq!(tag.user_id, user.id);
  assert_eq!(tag.category_id, category.id);
}

#[test]
fn test_edit_tag() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag = log::create_tag(
    "Original".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let edited = log::edit_tag(&tag.id, "Updated".to_string(), "red".to_string(), &user.id);

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(edited.name, "Updated");
  assert_eq!(edited.color, "red");
}

#[test]
fn test_get_tag() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag = log::create_tag(
    "Test Tag".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let found = log::get_tag(&tag.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, tag.id);
  assert_eq!(found.name, "Test Tag");
}

#[test]
fn test_get_tags() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag1 = log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  let tag2 = log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let tag_ids = vec![tag1.id.as_str(), tag2.id.as_str()];
  let tags = log::get_tags(tag_ids, &user.id);

  assert!(tags.is_ok());
  let tags = tags.unwrap();
  assert_eq!(tags.len(), 2);
}

#[test]
fn test_get_category_tags() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let tags = log::get_category_tags(&category.id, &user.id);

  assert!(tags.is_ok());
  let tags = tags.unwrap();
  assert_eq!(tags.len(), 2);
}

#[test]
fn test_delete_tag() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag = log::create_tag(
    "To Delete".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let deleted = log::delete_tag(&tag.id, &user.id);

  assert!(deleted.is_ok());
  assert_eq!(deleted.unwrap(), true);

  let found = log::get_tag(&tag.id, &user.id);
  assert!(found.is_err());
}

#[test]
fn test_delete_all_category_tags() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let deleted = log::delete_all_category_tags(&category.id, &user.id);

  assert!(deleted.is_ok());
  assert_eq!(deleted.unwrap(), true);

  let tags = log::get_category_tags(&category.id, &user.id);
  assert!(tags.is_ok());
  assert_eq!(tags.unwrap().len(), 0);
}

#[test]
fn test_create_entry() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag = log::create_tag(
    "Test Tag".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let entry = log::create_entry(
    "2025-10-17".to_string(),
    5,
    Some("Test entry content".to_string()),
    vec![tag.id.clone()],
    user.id.clone(),
  );

  assert!(entry.is_ok());
  let entry = entry.unwrap();
  assert_eq!(entry.date, "2025-10-17");
  assert_eq!(entry.mood, 5);
  assert_eq!(entry.entry, Some("Test entry content".to_string()));
  assert_eq!(entry.selected_tags.len(), 1);
  assert_eq!(entry.selected_tags[0], tag.id);
}

#[test]
fn test_create_entry_without_content() {
  let user = create_test_user();

  let entry = log::create_entry("2025-10-17".to_string(), 3, None, vec![], user.id.clone());

  assert!(entry.is_ok());
  let entry = entry.unwrap();
  assert_eq!(entry.entry, None);
  assert_eq!(entry.selected_tags.len(), 0);
}

#[test]
fn test_edit_entry() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag1 = log::create_tag(
    "Tag 1".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();
  let tag2 = log::create_tag(
    "Tag 2".to_string(),
    "red".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let entry = log::create_entry(
    "2025-10-17".to_string(),
    3,
    Some("Original content".to_string()),
    vec![tag1.id.clone()],
    user.id.clone(),
  )
  .unwrap();

  let edited = log::edit_entry(
    &entry.id,
    "2025-10-18".to_string(),
    4,
    Some("Updated content".to_string()),
    vec![tag2.id.clone()],
    &user.id,
  );

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(edited.date, "2025-10-18");
  assert_eq!(edited.mood, 4);
  assert_eq!(edited.entry, Some("Updated content".to_string()));
  assert_eq!(edited.selected_tags.len(), 1);
  assert_eq!(edited.selected_tags[0], tag2.id);
}

#[test]
fn test_get_entry_with_tags() {
  let user = create_test_user();
  let category = log::create_category("Test Category".to_string(), user.id.clone()).unwrap();
  let tag = log::create_tag(
    "Test Tag".to_string(),
    "blue".to_string(),
    category.id.clone(),
    user.id.clone(),
  )
  .unwrap();

  let entry = log::create_entry(
    "2025-10-17".to_string(),
    5,
    Some("Test entry".to_string()),
    vec![tag.id.clone()],
    user.id.clone(),
  )
  .unwrap();

  let found = log::get_entry_with_tags(&entry.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, entry.id);
  assert_eq!(found.selected_tags.len(), 1);
  assert_eq!(found.selected_tags[0], tag.id);
}

#[test]
fn test_create_default_data() {
  let user = create_test_user();

  let result = log::create_default_data(user.id.clone());

  assert!(result.is_ok());

  let categories = log::get_all_categories(&user.id).unwrap();
  assert!(categories.len() >= 2);

  let activities_cat = categories.iter().find(|c| c.name == "Activities");
  let tags_cat = categories.iter().find(|c| c.name == "Tags");

  assert!(activities_cat.is_some());
  assert!(tags_cat.is_some());

  if let Some(activities) = activities_cat {
    let tags = log::get_category_tags(&activities.id, &user.id).unwrap();
    assert!(tags.len() >= 6);
  }

  if let Some(tags) = tags_cat {
    let tag_list = log::get_category_tags(&tags.id, &user.id).unwrap();
    assert!(tag_list.len() >= 3);
  }
}
