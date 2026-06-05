/// Parameters - Controls flow of sending message
pub struct Parameters {
    pub schedule: bool,
    pub schedule_at: Option<time::OffsetDateTime>,
}
