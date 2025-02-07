#[derive(Debug)]
pub enum EphemerideError {
  Unauthorized,
  DatabaseError,
  InternalServerError,
  UserNotFound,
  InviteNotFound,
  SessionNotFound,
  EmailAlreadyInUse,
  InvalidPassword,
  InviteUsed,
}
