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
    let user = match user_store.get_user(&email).await {
        Ok(user) => { user },
        Err(_) => { return Err(AuthAPIError::IncorrectCredentials); },
    };
    Ok(StatusCode::OK.into_response())
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
