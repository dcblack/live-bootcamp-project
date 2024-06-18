use crate::{
  app_state::AppState,
  domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
  utils::auth::generate_auth_cookie,
  //routes::LoginResponse::TwoFactorAuth,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

const FAIL: &str = "[1;91m⚠ FAIL: [00m";
const NOTE: &str = "[1;96mNOTE: [00m";

pub async fn login(
  State(state): State<AppState>,
  jar: CookieJar,
  Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
  let password = match Password::parse(request.password) {
    Ok(password) => password,
    Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
  };
  let email = match Email::parse(request.email) {
    Ok(email) => email,
    Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
  };
  let user_store = &state.user_store.read().await;
  if user_store.validate_user(&email, &password).await.is_err() {
    return (jar, Err(AuthAPIError::IncorrectCredentials));
  }

  let user = match user_store.get_user(&email).await {
    Ok(user) => user,
    Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
  };

  // Handle request based on user's 2FA configuration
  match user.requires_2fa {
    true => handle_2fa(&user.email, &state, jar).await,
    false => handle_no_2fa(&user.email, jar).await,
  }
}

async fn handle_2fa(
  email: &Email,
  state: &AppState,
  jar: CookieJar,
) -> (
  CookieJar,
  Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
  let login_attempt_id = LoginAttemptId::default();
  let two_fa_code = TwoFACode::default();
  if let Err(_e) = state
    .two_fa_code_store
    .write()
    .await
    .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
    .await
  {
    println!("{FAIL}Unable to add 2FA code for {:?}", email);
    return (jar, Err(AuthAPIError::UnexpectedError));
  }
  println!("{NOTE}Added 2FA code for {:?}", email);
  if let Err(_e) = state
    .email_client
    .write()
    .await
    .send_email(&email, "Verify login", two_fa_code.as_ref())
    .await
  {
    println!("{FAIL}Unable to send email to {:?}", email);
    return (jar, Err(AuthAPIError::UnexpectedError));
  }
  let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
    message: "2FA required".to_owned(),
    login_attempt_id: login_attempt_id.as_ref().to_owned(),
  }));
  (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

// New!
async fn handle_no_2fa(
  email: &Email,
  jar: CookieJar,
) -> (
  CookieJar,
  Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
  let auth_cookie = match generate_auth_cookie(email) {
    Ok(cookie) => cookie,
    Err(_) => {
      println!("{FAIL}Unable to generate cookie for {:?}", email);
      return (jar, Err(AuthAPIError::UnexpectedError));
    }
  };

  let updated_jar = jar.add(auth_cookie);

  (
    updated_jar,
    Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
  )
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
  pub message: String,
  #[serde(rename = "loginAttemptId")]
  pub login_attempt_id: String,
}

#[derive(Deserialize, Clone)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
  RegularAuth,
  TwoFactorAuth(TwoFactorAuthResponse),
}
