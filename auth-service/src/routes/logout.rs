use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
  app_state::AppState,
  domain::AuthAPIError,
  utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
  State(state): State<AppState>,
  jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
  // Retrieve JWT cookie from the `CookieJar`
  // Return AuthAPIError::MissingToken is the cookie is not found
  let cookie = match jar.get(JWT_COOKIE_NAME) {
    Some(cookie) => cookie,
    None => return (jar, Err(AuthAPIError::MissingToken)),
  };

  let token = cookie.value().to_owned();

  // Validate the token
  if let Err(_error) = validate_token(token.as_str(), state.banned_token_store.clone()).await {
    return (jar, Err(AuthAPIError::InvalidToken));
  }

  // Remove the cookie
  let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

  // Poison the token
  let result = state.banned_token_store.write().await.insert(&token).await;
  if let Err(_error) = result {
    return (jar, Err(AuthAPIError::InvalidToken));
  }

  (jar, Ok(StatusCode::OK))
}
