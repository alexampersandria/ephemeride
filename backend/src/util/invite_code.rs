use uuid::Uuid;

// #TODO: generate more human readable but still unique invite codes
pub fn generate_invite_code() -> String {
  Uuid::new_v4().to_string()
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn test_generate_invite_code_uniqueness() {
    let code1 = generate_invite_code();
    let code2 = generate_invite_code();
    assert_ne!(code1, code2);
  }
}
