use std::time::Duration;

pub fn duration_to_string(duration: &Duration) -> String {
    let s = duration.as_secs();
    let m = (s / 60) % 60;
    let h = s / 3600;
    let s = s % 60;
    format!("{}h {:0>2}m {:0>2}s", h, m, s)
}
