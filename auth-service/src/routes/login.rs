use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{
  app_state::AppState,
  domain::{AuthAPIError, Email, Password, LoginAttemptId, TwoFACode},
  utils::auth::generate_auth_cookie,
  //routes::LoginResponse::TwoFactorAuth,
};

const WARN: &str = "[1;91m⚠ WARNING: [00m";

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
    true => handle_2fa(&user.email,&state, jar).await,
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
    .two_fa_code_store.write()
    .await
    .add_code(email.clone(),login_attempt_id.clone(),two_fa_code.clone())
    .await
  {
    println!("{WARN}Unable to add code {:?}", email);
    return (jar, Err(AuthAPIError::UnexpectedError));
  }
  if let Err(_e) = state.email_client
    .write().await
    .send_email(&email,"Verify login",two_fa_code.as_ref())
    .await
  {
    println!("{WARN}Unable to send email {:?}", email);
    return (jar, Err(AuthAPIError::UnexpectedError));
  }
  let response = Json(LoginResponse::TwoFactorAuth(
    TwoFactorAuthResponse{
      message: "2FA required".to_owned(),
      login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }
  ));
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
      println!("{WARN}Unable to generate cookie {:?}", email);
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