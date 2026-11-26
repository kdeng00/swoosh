pub const DEFAULT_SCHEDULING_SECONDS: i64 = 300;

pub fn is_scheduleable(
    now: Option<&time::OffsetDateTime>,
    scheduled: Option<&time::OffsetDateTime>,
) -> bool {
    match scheduled {
        Some(schedule_at) => {
            let early = now
                .unwrap()
                .checked_add(time::Duration::seconds(DEFAULT_SCHEDULING_SECONDS))
                .unwrap();
            *schedule_at > early
        }
        None => false,
    }
}
