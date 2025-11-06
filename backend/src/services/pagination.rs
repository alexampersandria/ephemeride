use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationObject {
  pub limit: i64,
  pub offset: i64,
  pub total_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paginated<T> {
  pub data: Vec<T>,
  pub pagination: PaginationObject,
}
