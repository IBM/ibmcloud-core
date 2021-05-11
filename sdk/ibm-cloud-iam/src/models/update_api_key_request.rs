/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateApiKeyRequest : Input body parameters for the Update API key V1 REST request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateApiKeyRequest {
    /// The name of the API key to update. If specified in the request the parameter must not be empty. The name is not checked for uniqueness. Failure to this will result in an Error condition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the API key to update. If specified an empty description will clear the description of the API key. If a non empty value is provided the API key will be updated.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateApiKeyRequest {
    /// Input body parameters for the Update API key V1 REST request.
    pub fn new() -> UpdateApiKeyRequest {
        UpdateApiKeyRequest {
            name: None,
            description: None,
        }
    }
}


