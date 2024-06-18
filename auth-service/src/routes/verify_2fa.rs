use crate::{
  app_state::AppState,
  domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
  utils::auth::generate_auth_cookie,
};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

const FAIL: &str = "[1;91m⚠ FAIL: [00m";

pub async fn verify_2fa(
  State(state): State<AppState>,
  jar: CookieJar,
  Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
  // Check email
  let email = match Email::parse(request.email) {
    Ok(email) => email,
    Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
  };
  // Check login_attempt_id
  let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
    Ok(login_attempt_id) => login_attempt_id,
    Err(_) => return (jar, Err(AuthAPIError::InvalidLoginAttempt)),
  };
  // Check two_fa_code
  if let Err(_) = TwoFACode::parse(request.two_fa_code.clone()) {
    return (jar, Err(AuthAPIError::Invalid2FACode));
  };

  let mut two_fa_code_store = state.two_fa_code_store.write().await;
  let attempt_n_code = match two_fa_code_store.get_2fa_code(&email).await {
    Ok(attempt_n_code) => attempt_n_code,
    Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
  };

  let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
    Ok(two_fa_code) => two_fa_code,
    Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
  };

  // Compare codes
  if !attempt_n_code.0.eq(&login_attempt_id) || !attempt_n_code.1.eq(&two_fa_code) {
    return (jar, Err(AuthAPIError::IncorrectCredentials));
  }

  if two_fa_code_store.remove_code(&email).await.is_err() {
    println!("{FAIL}Unable to remove code {:?}", email);
    return (jar, Err(AuthAPIError::UnexpectedError));
  }

  let cookie = match generate_auth_cookie(&email) {
    Ok(cookie) => cookie,
    Err(_) => {
      println!("{FAIL}Unable to generate auth cookie {:?}", email);
      return (jar, Err(AuthAPIError::UnexpectedError));
    }
  };

  let updated_jar = jar.add(cookie);

  (updated_jar, Ok(()))
}

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
  pub email: String,
  #[serde(rename = "loginAttemptId")]
  pub login_attempt_id: String,
  #[serde(rename = "2FACode")]
  pub two_fa_code: String,
}
