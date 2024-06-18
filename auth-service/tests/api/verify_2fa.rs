use auth_service::domain::TwoFACodeStore;
use auth_service::{
  domain::{Email, LoginAttemptId, TwoFACode},
  routes::TwoFactorAuthResponse,
  //utils::constants::JWT_COOKIE_NAME,
  ErrorResponse,
};

use crate::helpers::{
  get_random_email,
  TestApp,
  GREAT_PASSWORD,
  //WRONG_PASSWORD,
  //EMPTY_PASSWORD,
  //-- Debug help
  //ALERT,
  //NOTE,
};

#[tokio::test]
async fn verify_2fa_should_return_400_if_invalid_input() {
  let app = TestApp::new().await;

  let input = [serde_json::json!({
      "email": "",
      "loginAttemptId": "",
      "2FACode": "",
  })];

  for body in input.iter() {
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(
      response.status().as_u16(),
      400,
      "Failed for input: {:?}",
      body
    );
  }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
  let app = TestApp::new().await;
  let random_email = get_random_email();
  let signup_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
      "requires2FA": true
  });
  let response = app.post_signup(&signup_body).await;
  assert_eq!(response.status().as_u16(), 201);
  let wrong_email = get_random_email();
  let wrong_login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
  let wrong_two_fa_code = TwoFACode::default().as_ref().to_owned();

  let test_cases = vec![
    (
      wrong_email.as_str(),
      wrong_login_attempt_id.as_str(),
      wrong_two_fa_code.clone(),
    ),
    (
      random_email.as_str(),
      wrong_login_attempt_id.as_str(),
      wrong_two_fa_code.clone(),
    ),
    (
      random_email.as_str(),
      wrong_login_attempt_id.as_str(),
      wrong_two_fa_code.clone(),
    ),
  ];

  for (email, login_attempt_id, code) in test_cases {
    let request_body = serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(
      response.status().as_u16(),
      401,
      "Failed for input: {:?}",
      request_body
    );

    assert_eq!(
      response
        .json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse")
        .error,
      "Incorrect credentials".to_owned()
    );
  }
}

#[tokio::test]
async fn should_return_401_if_old_code() {
  // Call login twice. Then, attempt to call verify-fa with the 2FA code
  // from the first login request. This should fail.
  let app = TestApp::new().await;

  let random_email = get_random_email();

  let signup_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD,
      "requires2FA": true
  });

  let response = app.post_signup(&signup_body).await;

  assert_eq!(response.status().as_u16(), 201);

  // First login call

  let login_body = serde_json::json!({
      "email": random_email,
      "password": GREAT_PASSWORD
  });

  //println!("{NOTE}First login for {:?}", random_email.clone());
  let response = app.post_login(&login_body).await;

  assert_eq!(response.status().as_u16(), 206);

  let response_body = response
    .json::<TwoFactorAuthResponse>()
    .await
    .expect("Could not deserialize response body to TwoFactorAuthResponse");

  assert_eq!(response_body.message, "2FA required".to_owned());
  assert!(!response_body.login_attempt_id.is_empty());

  let login_attempt_id = response_body.login_attempt_id;

  let attempt_n_code = app
    .two_fa_code_store
    .read()
    .await
    .get_2fa_code(&Email::parse(random_email.clone()).unwrap())
    .await
    .unwrap();

  let first_code = attempt_n_code.1.as_ref();

  // Second login call
  //println!("{NOTE}Second login for {:?}", random_email.clone());

  let response = app.post_login(&login_body).await;

//println!("{ALERT}Known bug - should be 206");
  assert_eq!(response.status().as_u16(), 206);

  // 2FA attempt with old login_attempt_id and first code

  let request_body = serde_json::json!({
      "email": random_email.clone(),
      "loginAttemptId": login_attempt_id,
      "2FACode": first_code
  });

  //println!("{NOTE}Verify 2FA for {:?}", random_email.clone());
  let response = app.post_verify_2fa(&request_body).await;

//println!("{ALERT}Known bug - should be 401 (fix 500 above)");
  assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn verify_2fa_should_return_422_if_malformed_input() {
  let app = TestApp::new().await;

  let verify_body = serde_json::json!({
      "junk": "",
  });

  let response = app.post_verify_2fa(&verify_body).await;
  assert_eq!(response.status().as_u16(), 422, "Failed for input: ");
}
