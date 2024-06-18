use std::error::Error;

use app_state::AppState;
use axum::{
  http::{Method, StatusCode},
  response::{IntoResponse, Response},
  routing::post,
  serve::Serve,
  Json, Router,
};
use domain::AuthAPIError;
use routes::{login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
  server: Serve<Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
    let allowed_origins = [
      "http://localhost:8000".parse()?,
      "http://206.81.0.37:8000".parse()?,
    ];
    let cors = CorsLayer::new()
      // Allow GET and POST requests
      .allow_methods([Method::GET, Method::POST])
      // Allow cookies to be included in requests
      .allow_credentials(true)
      .allow_origin(allowed_origins);
    let router = Router::new()
      .nest_service("/", ServeDir::new("assets"))
      .route("/signup", post(signup))
      .route("/login", post(login))
      .route("/verify-2fa", post(verify_2fa))
      .route("/logout", post(logout))
      .route("/verify-token", post(verify_token))
      .with_state(app_state)
      .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
    let server = axum::serve(listener, router);

    Ok(Application { server, address })
  }

  pub async fn run(self) -> Result<(), std::io::Error> {
    println!("listening on {}", &self.address);
    self.server.await
  }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
  pub error: String,
}

impl IntoResponse for AuthAPIError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"), //409
      AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"), //400
      AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"), //401
      AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"), //500
      AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"), //400
      AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"), //401
      AuthAPIError::InvalidLoginAttempt => (StatusCode::BAD_REQUEST, "Invalid login attempt"), //400
      AuthAPIError::Invalid2FACode => (StatusCode::BAD_REQUEST, "Invalid login attempt"), //400
    };
    let body = Json(ErrorResponse {
      error: error_message.to_string(),
    });
    (status, body).into_response()
  }
}
