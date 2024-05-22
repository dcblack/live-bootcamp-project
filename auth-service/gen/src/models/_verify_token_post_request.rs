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
pub struct VerifyTokenPostRequest {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl VerifyTokenPostRequest {
    pub fn new() -> VerifyTokenPostRequest {
        VerifyTokenPostRequest {
            token: None,
        }
    }
}

