use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_ms() -> i64 {
  let duration_since_unix_epoch = SystemTime::now().duration_since(UNIX_EPOCH);
  if let Ok(duration) = duration_since_unix_epoch {
    duration.as_millis() as i64
  } else {
    0
  }
}
