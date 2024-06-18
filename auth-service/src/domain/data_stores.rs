use super::{Email, Password, User};
use rand::Rng;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserStore {
  async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
  async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
  async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

//------------------------------------------------------------------------------
#[async_trait::async_trait]
pub trait BannedTokenStore {
  async fn insert(&mut self, token_data: &str) -> Result<(), BannedTokenStoreError>;
  async fn is_banned(&self, token_data: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
  AlreadyBanned,
  Unexpected,
}

//------------------------------------------------------------------------------
// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
  async fn add_code(
    &mut self,
    email: Email,
    login_attempt_id: LoginAttemptId,
    code: TwoFACode,
  ) -> Result<(), TwoFACodeStoreError>;
  async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
  async fn get_2fa_code(
    &self,
    email: &Email,
  ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
  LoginAttemptIdNotFound,
  UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
  pub fn parse(id: String) -> Result<Self, String> {
    // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
    let uuid = Uuid::try_parse(id.as_str()).map_err(|_| "Invalid login attempt id".to_owned())?;
    Ok(Self(uuid.to_string()))
  }
}

impl Default for LoginAttemptId {
  fn default() -> Self {
    // Use the `uuid` crate to generate a random version 4 UUID
    Self(Uuid::new_v4().to_string())
  }
}

impl AsRef<str> for LoginAttemptId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
  pub fn parse(code: String) -> Result<Self, String> {
    let code_as_u32 = code
      .parse::<u32>()
      .map_err(|_| "Invalid 2FA code".to_owned())?;

    if (100_000..=999_999).contains(&code_as_u32) {
      Ok(Self(code))
    } else {
      Err("Invalid 2FA code".to_owned())
    }
  }
}

impl Default for TwoFACode {
  fn default() -> Self {
    Self(rand::thread_rng().gen_range(100_000..=999_999).to_string())
  }
}

impl AsRef<str> for TwoFACode {
  fn as_ref(&self) -> &str {
    &self.0
  }
}
