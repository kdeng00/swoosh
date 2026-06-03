#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TwilioAPIMessage {
    pub account_sid: String,
    pub api_version: String,
    pub body: String,
    // pub date_created: time::OffsetDateTime,
    pub date_sent: Option<time::OffsetDateTime>,
    pub date_updated: Option<time::OffsetDateTime>,
    pub direction: String,
    // Not definite of what type this is
    pub error_code: Option<String>,
    // Not definite of what type this is
    pub error_message: Option<String>,
    pub from: String,
    pub messaging_service_sid: String,
    pub num_media: String,
    pub num_segments: String,
    pub price: Option<String>,
    pub price_unit: Option<String>,
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

/*
{
    "account_sid": "ACefa1ef516314c9d1a68cbd657de49277",
    "api_version": "2010-04-01",
    "body": "This is a test",
    "date_created": "Wed, 03 Jun 2026 19:40:55 +0000",
    "date_sent": null,
    "date_updated": "Wed, 03 Jun 2026 19:40:55 +0000",
    "direction": "outbound-api",
    "error_code": null,
    "error_message": null,
    "from": "+12243026041",
    "messaging_service_sid": "MG803f3676706b92eb02e18dd820c447f2",
    "num_media": "0",
    "num_segments": "0",
    "price": null,
    "price_unit": null,
    "sid": "SMf5c09c776c931bdb00c6e274da2ce34d",
    "status": "scheduled",
    "subresource_uris": {
        "media": "/2010-04-01/Accounts/ACefa1ef516314c9d1a68cbd657de49277/Messages/SMf5c09c776c931bdb00c6e274da2ce34d/Media.json"
    },
    "to": "+16303831708",
    "uri": "/2010-04-01/Accounts/ACefa1ef516314c9d1a68cbd657de49277/Messages/SMf5c09c776c931bdb00c6e274da2ce34d.json"
}
*/
