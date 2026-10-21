/// Whether a snapshot taken at `snapshot_at` is still fresh at `now`.
///
/// Fresh when `now` is at least `snapshot_at` and the elapsed seconds are less
/// than or equal to `max_age_secs`. A `now` before the snapshot is not fresh.
/// An age equal to `max_age_secs` is still fresh.
pub fn is_fresh(snapshot_at: u64, now: u64, max_age_secs: u64) -> bool {
    match now.checked_sub(snapshot_at) {
        Some(age) => age <= max_age_secs,
        None => false,
    }
}
