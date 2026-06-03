// Things needed to send a message
// Contact - Receipient to send to (textsender-models)
// Message - The content to sent (textsender-models)
// Parameters - Controls flow of sending message
// Config - Contains info needed to send message (textsender-models)
pub struct Parameters {
    pub schedule: bool,
    pub schedule_at: Option<time::OffsetDateTime>,
}
