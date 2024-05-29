use axum::{http::StatusCode, Json, response::IntoResponse};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, crate::domain::error::AuthAPIError > {
    let email =
      Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
      Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let user_store = state.user_store.read().await;
    match user_store.get_user(&email).await {
        Err(_) => { return Err(AuthAPIError::InvalidCredentials); },
        Ok(user) => {
            // Check request against the password
            if user.password == password {
            if user.requires_2fa {
                let response = Json(LoginResponse {
                    message: "Login successful!".to_string(),
                });
                return Ok((StatusCode::ACCEPTED, response));
            } else {
                let response = Json(LoginResponse {
                    message: "Login successful!".to_string(),
                });
                return Ok((StatusCode::ACCEPTED, response));
            }
          } else {
            return Err(AuthAPIError::InvalidCredentials);
          }
        }
    }
}

#[derive(Deserialize,Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}
