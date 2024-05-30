use auth_service::ErrorResponse;
use auth_service::routes::SignupResponse;
use crate::helpers::{get_random_email, TestApp};

use crate::helpers::{
    GREAT_PASSWORD,
    WRONG_PASSWORD,
    SHORT_PASSWORD,
    EMPTY_PASSWORD,
};


/* NO LONGER RELEVANT FROM SPRINT 4 ONWARDS
#[tokio::test]
async fn login_should_return_200() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let login_body = serde_json::json!({
        "email": random_email,
        "password": GREAT_PASSWORD,
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
}
*/
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
/*
*/

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
/*
    // Create a user to test against wrong password
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": GREAT_PASSWORD,
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };
    assert_eq!(
        response
          .json::<SignupResponse>()
          .await
          .expect("Could not deserialize response body to UserBody"),
        expected_response
    );

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

    // Test against wrong password

*/}