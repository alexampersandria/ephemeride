use uuid::Uuid;

pub fn generate_invite_code() -> String {
  let generated_code = Uuid::new_v4().to_string();
  generated_code
}
