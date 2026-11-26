pub fn convert_time_to_iso(time: time::OffsetDateTime) -> Result<String, time::error::Format> {
    use time::format_description::well_known::Iso8601;
    time.format(&Iso8601::DEFAULT)
}
