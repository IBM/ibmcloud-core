/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContextStateless {
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<Box<crate::models::MessageContextGlobalStateless>>,
    /// Information specific to particular skills used by the assistant.  **Note:** Currently, only a single child property is supported, containing variables that apply to the dialog skill used by the assistant.
    #[serde(rename = "skills", skip_serializing_if = "Option::is_none")]
    pub skills: Option<::std::collections::HashMap<String, crate::models::MessageContextSkill>>,
}

impl MessageContextStateless {
    pub fn new() -> MessageContextStateless {
        MessageContextStateless {
            global: None,
            skills: None,
        }
    }
}

