use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
  domain::{AuthAPIError},
  utils::{
    auth::validate_token,
  },
};

pub async fn verify_token(
  Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {

  let token = request.token.to_owned();

  // Validate the token
  if let Err(_error) = validate_token(token.as_str()).await {
    return Err(AuthAPIError::InvalidToken);
  }

  Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct VerifyTokenRequest {
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VerifyTokenResponse {
  pub message: String,
}
