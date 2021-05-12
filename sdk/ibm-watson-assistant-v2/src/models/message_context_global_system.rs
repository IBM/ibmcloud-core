/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MessageContextGlobalSystem : Built-in system properties that apply to all skills used by the assistant.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContextGlobalSystem {
    /// The user time zone. The assistant uses the time zone to correctly resolve relative time references.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// A string value that identifies the user who is interacting with the assistant. The client must provide a unique identifier for each individual end user who accesses the application. For user-based plans, this user ID is used to identify unique users for billing purposes. This string cannot contain carriage return, newline, or tab characters. If no value is specified in the input, **user_id** is automatically set to the value of **context.global.session_id**.  **Note:** This property is the same as the **user_id** property at the root of the message body. If **user_id** is specified in both locations in a message request, the value specified at the root is used.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// A counter that is automatically incremented with each turn of the conversation. A value of 1 indicates that this is the the first turn of a new conversation, which can affect the behavior of some skills (for example, triggering the start node of a dialog).
    #[serde(rename = "turn_count", skip_serializing_if = "Option::is_none")]
    pub turn_count: Option<i32>,
    /// The language code for localization in the user input. The specified locale overrides the default for the assistant, and is used for interpreting entity values in user input such as date values. For example, `04/03/2018` might be interpreted either as April 3 or March 4, depending on the locale.   This property is included only if the new system entities are enabled for the skill.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<Locale>,
    /// The base time for interpreting any relative time mentions in the user input. The specified time overrides the current server time, and is used to calculate times mentioned in relative terms such as `now` or `tomorrow`. This can be useful for simulating past or future times for testing purposes, or when analyzing documents such as news articles.  This value must be a UTC time value formatted according to ISO 8601 (for example, `2019-06-26T12:00:00Z` for noon on 26 June 2019.  This property is included only if the new system entities are enabled for the skill.
    #[serde(rename = "reference_time", skip_serializing_if = "Option::is_none")]
    pub reference_time: Option<String>,
}

impl MessageContextGlobalSystem {
    /// Built-in system properties that apply to all skills used by the assistant.
    pub fn new() -> MessageContextGlobalSystem {
        MessageContextGlobalSystem {
            timezone: None,
            user_id: None,
            turn_count: None,
            locale: None,
            reference_time: None,
        }
    }
}

/// The language code for localization in the user input. The specified locale overrides the default for the assistant, and is used for interpreting entity values in user input such as date values. For example, `04/03/2018` might be interpreted either as April 3 or March 4, depending on the locale.   This property is included only if the new system entities are enabled for the skill.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "en-ca")]
    EnCa,
    #[serde(rename = "en-gb")]
    EnGb,
    #[serde(rename = "ar-ar")]
    ArAr,
    #[serde(rename = "cs-cz")]
    CsCz,
    #[serde(rename = "de-de")]
    DeDe,
    #[serde(rename = "es-es")]
    EsEs,
    #[serde(rename = "fr-fr")]
    FrFr,
    #[serde(rename = "it-it")]
    ItIt,
    #[serde(rename = "ja-jp")]
    JaJp,
    #[serde(rename = "ko-kr")]
    KoKr,
    #[serde(rename = "nl-nl")]
    NlNl,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-tw")]
    ZhTw,
}
