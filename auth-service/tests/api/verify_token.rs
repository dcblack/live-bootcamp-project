use crate::helpers::{get_random_email, TestApp, GREAT_PASSWORD};
use auth_service::utils::constants::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_200_valid_token() {
  let app = TestApp::new().await;

  // Signup
  let random_email = get_random_email();
  let signup_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
      "requires2FA": false
  });
  let response = app.post_signup(&signup_body).await;
  assert_eq!(response.status().as_u16(), 201);

  // Login
  let login_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
  });
  let response = app.post_login(&login_body).await;
  assert_eq!(response.status().as_u16(), 200);

  // Get the token should stored in cookie
  let auth_cookie = response
    .cookies()
    .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
    .expect("No auth cookie found");
  assert!(!auth_cookie.value().is_empty());
  let token = auth_cookie.value();

  let verify_body = serde_json::json!({ "token": token });
  let response = app.post_verify_token(&verify_body).await;
  assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
  let app = TestApp::new().await;

  let random_email = get_random_email();
  let signup_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
      "requires2FA": false
  });

  let response = app.post_signup(&signup_body).await;
  assert_eq!(response.status().as_u16(), 201);

  let login_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
  });

  // Login
  let response = app.post_login(&login_body).await;
  assert_eq!(response.status().as_u16(), 200);

  let auth_cookie = response
    .cookies()
    .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
    .expect("No auth cookie found");
  assert!(!auth_cookie.value().is_empty());

  let token = auth_cookie.value();

  // Logout
  let response = app.post_logout().await;
  assert_eq!(response.status().as_u16(), 200);

  // // Ban to see what happens
  // let mut banned_token_store = app.banned_token_store.write().await;
  // let banned = banned_token_store
  //   .insert(token)
  //   .await;
  // //assert!(banned.is_ok());

  let verify_body = serde_json::json!({ "token": token });
  let response = app.post_verify_token(&verify_body).await;
  assert_eq!(response.status().as_u16(), 401);
}
#[tokio::test]
async fn should_return_401_if_invalid_token() {
  let app = TestApp::new().await;

  let verify_body = serde_json::json!({
      "token": "",
  });

  let response = app.post_verify_token(&verify_body).await;

  assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
  let app = TestApp::new().await;

  let verify_body = serde_json::json!({
      "Xoken": "",
  });

  let response = app.post_verify_token(&verify_body).await;

  assert_eq!(response.status().as_u16(), 422);
}
