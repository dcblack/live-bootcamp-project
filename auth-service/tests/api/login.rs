use auth_service::ErrorResponse;
use crate::helpers::{get_random_email, TestApp};

use crate::helpers::{
    GREAT_PASSWORD,
    //WRONG_PASSWORD,
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

    let random_email = get_random_email();

    let input = [
        serde_json::json!({
            "email": "", // empty email
            "password": GREAT_PASSWORD,
        }),
        serde_json::json!({
            "email": random_email,
            "password": EMPTY_PASSWORD, // empty password
        }),
        serde_json::json!({
            "email": "",    // empty email
            "password": EMPTY_PASSWORD, // AND empty password
        }),
        serde_json::json!({
            "email": "invalid_email", // missing @domain
            "password": GREAT_PASSWORD,
        }),
        serde_json::json!({
            "email": random_email,
            "password": SHORT_PASSWORD, // too short password
        }),
    ];

    for i in input.iter() {
        let response = app.post_login(i).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", i);

        assert_eq!(
            response
              .json::<ErrorResponse>()
              .await
              .expect("Could not deserialize response body to ErrorResponse")
              .error,
            "Invalid credentials".to_owned()
        );
    }
}
