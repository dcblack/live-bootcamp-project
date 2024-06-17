//use serde::{Deserialize, Serialize};
use auth_service::{
  utils::constants::JWT_COOKIE_NAME,
  //domain::{Email, TwoFACode, },
  routes::TwoFactorAuthResponse,
  //ErrorResponse,
};
use crate::helpers::{
  get_random_email,
  TestApp,
  GREAT_PASSWORD,
  WRONG_PASSWORD,
  EMPTY_PASSWORD,
};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
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

  let response = app.post_login(&login_body).await;

  assert_eq!(response.status().as_u16(), 200);

  let auth_cookie = response
    .cookies()
    .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
    .expect("No auth cookie found");

  assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
  let app = TestApp::new().await;

  let random_email = get_random_email();

  let signup_body = serde_json::json!({
        "email": random_email,
        "password": GREAT_PASSWORD,
        "requires2FA": true
    });

  let response = app.post_signup(&signup_body).await;

  assert_eq!(response.status().as_u16(), 201);

  let login_body = serde_json::json!({
        "email": random_email,
        "password": GREAT_PASSWORD,
    });

  let response = app.post_login(&login_body).await;

  assert_eq!(response.status().as_u16(), 206);
  let json_body = response
    .json::<TwoFactorAuthResponse>()
    .await
    .expect("Could not serialize response body to TwoFactorAuthResponse");
  assert_eq!(json_body.message, "2FA required".to_owned());
  // let auth_cookie = response
  //   .cookies()
  //   .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
  //   .expect("No auth cookie found");
  //
  // assert!(!auth_cookie.value().is_empty());
}


#[tokio::test]
async fn should_return_400_if_invalid_input() {
  // Call the log-in route with invalid credentials and assert that a
  // 400 HTTP status code is returned along with the appropriate error message.
  let app = TestApp::new().await;

  let random_email = get_random_email();

  let input = [
    serde_json::json!({
            "email": "", // empty email
            "password": EMPTY_PASSWORD, // empty password
        }),
    serde_json::json!({
            "email": "", // empty email
            "password": GREAT_PASSWORD,
        }),
    serde_json::json!({
            "email": random_email,
            "password": EMPTY_PASSWORD, // empty password
        }),
  ];
  for i in input.iter() {
    let response = app.post_login(i).await;
    assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);
  }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
  // Call the log-in route with incorrect credentials and assert
  // that a 401 HTTP status code is returned along with the appropriate error message.
  let app = TestApp::new().await;

  let random_email = get_random_email();

  let input = [
    serde_json::json!({
            "email": random_email, // missing @domain
            "password": WRONG_PASSWORD,
        }),
  ];
  for i in input.iter() {
    let response = app.post_login(i).await;
    assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", i);
  }

}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
  let app = TestApp::new().await;

  let input = [
    serde_json::json!({
        }),
  ];

  for i in input.iter() {
    let response = app.post_login(i).await;
    assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", i);
  }
}

