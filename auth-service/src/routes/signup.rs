use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

const FAIL: &str = "[1;91m⚠ FAIL: [00m";

use crate::{
  app_state::AppState,
  domain::{AuthAPIError, Email, Password, User},
};

pub async fn signup(
  State(state): State<AppState>,
  Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
  let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
  let password =
    Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

  let user = User::new(email, password, request.requires_2fa);

  let mut user_store = state.user_store.write().await;

  if user_store.get_user(&user.email).await.is_ok() {
    return Err(AuthAPIError::UserAlreadyExists);
  }

  if user_store.add_user(user.clone()).await.is_err() {
    println!("{FAIL}Unable to add user for {:?}", &user.email);
    return Err(AuthAPIError::UnexpectedError);
  }
  let response = Json(SignupResponse {
    message: "User created successfully!".to_string(),
  });
  Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
  pub email: String,
  pub password: String,
  #[serde(rename = "requires2FA")]
  pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
  pub message: String,
}
