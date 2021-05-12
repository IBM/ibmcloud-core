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
pub struct Log {
    /// A unique identifier for the logged event.
    #[serde(rename = "log_id")]
    pub log_id: String,
    #[serde(rename = "request")]
    pub request: Box<crate::models::MessageRequest>,
    #[serde(rename = "response")]
    pub response: Box<crate::models::MessageResponse>,
    /// Unique identifier of the assistant.
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    /// The ID of the session the message was part of.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// The unique identifier of the skill that responded to the message.
    #[serde(rename = "skill_id")]
    pub skill_id: String,
    /// The name of the snapshot (dialog skill version) that responded to the message (for example, `draft`).
    #[serde(rename = "snapshot")]
    pub snapshot: String,
    /// The timestamp for receipt of the message.
    #[serde(rename = "request_timestamp")]
    pub request_timestamp: String,
    /// The timestamp for the system response to the message.
    #[serde(rename = "response_timestamp")]
    pub response_timestamp: String,
    /// The language of the assistant to which the message request was made.
    #[serde(rename = "language")]
    pub language: String,
    /// The customer ID specified for the message, if any.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}

impl Log {
    pub fn new(log_id: String, request: crate::models::MessageRequest, response: crate::models::MessageResponse, assistant_id: String, session_id: String, skill_id: String, snapshot: String, request_timestamp: String, response_timestamp: String, language: String) -> Log {
        Log {
            log_id,
            request: Box::new(request),
            response: Box::new(response),
            assistant_id,
            session_id,
            skill_id,
            snapshot,
            request_timestamp,
            response_timestamp,
            language,
            customer_id: None,
        }
    }
}

