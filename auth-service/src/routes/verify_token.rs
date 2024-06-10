use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
  services::hashset_banned_token_store::HashsetBannedTokenStore,
  domain::{AuthAPIError},
  utils::{
    auth::validate_token,
  },
};
use crate::app_state::AppState;

pub async fn verify_token(
  State(state): State<AppState>,
  Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {

  let token = request.token.to_owned();

  // Validate the token
  if let Err(_error) = validate_token(token.as_str(),state.banned_token_store.clone()).await {
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
