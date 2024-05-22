/*
 * Authentication Service API
 *
 * This is an API for an authentication service using JWT and optional email 2FA.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoginPostRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl LoginPostRequest {
    pub fn new() -> LoginPostRequest {
        LoginPostRequest {
            email: None,
            password: None,
        }
    }
}

