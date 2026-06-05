#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TwilioAPIMessage {
    pub account_sid: String,
    pub api_version: String,
    pub body: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_created: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_sent: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_updated: Option<time::OffsetDateTime>,
    pub direction: String,
    // Not definite of what type this is
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error_code: String,
    // Not definite of what type this is
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error_message: String,
    pub from: String,
    pub messaging_service_sid: String,
    pub num_media: String,
    pub num_segments: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub price: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub price_unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sid: String,
    pub status: String,
    pub subresource_uris: SubresourceURI,
    pub to: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SubresourceURI {
    pub media: String,
}
