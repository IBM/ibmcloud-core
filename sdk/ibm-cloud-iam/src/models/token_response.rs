/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TokenResponse : Response body for POST /identity/token.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenResponse {
    /// The IAM access token that can be used to invoke various IBM Cloud APIs. Use this token with the prefix Bearer in the HTTP header Authorization for invocations of IAM compatible APIs.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// (optional) A refresh token that can be used to get a new IAM access token if that token is expired. When using the default client (no basic authorization header) as described in this documentation, this refresh_token cannot be used to retrieve a new IAM access token. When the IAM access token is about to be expired, use the API key to create a new access token.
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// (optional) A delegated refresh token that can only be consumed by the clients that have been specified in the API call as 'receiver_client_ids'
    #[serde(rename = "delegated_refresh_token", skip_serializing_if = "Option::is_none")]
    pub delegated_refresh_token: Option<String>,
    /// The type of the token. Currently, only Bearer is returned.
    #[serde(rename = "token_type", skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    /// Number of seconds until the IAM access token will expire.
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    /// Number of seconds counted since January 1st, 1970, until the IAM access token will expire.
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i32>,
}

impl TokenResponse {
    /// Response body for POST /identity/token.
    pub fn new() -> TokenResponse {
        TokenResponse {
            access_token: None,
            refresh_token: None,
            delegated_refresh_token: None,
            token_type: None,
            expires_in: None,
            expiration: None,
        }
    }
}


