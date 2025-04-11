//! Utility functions for Zenobuf

use std::time::Duration;

use crate::time::Time;

/// Sleeps for the given duration
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Returns the current time
pub fn now() -> Time {
    Time::now()
}

/// Converts a Duration to a human-readable string
pub fn duration_to_string(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s {}ms", hours, minutes, seconds, millis)
    } else if minutes > 0 {
        format!("{}m {}s {}ms", minutes, seconds, millis)
    } else if seconds > 0 {
        format!("{}s {}ms", seconds, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Converts a string to a Duration
pub fn string_to_duration(s: &str) -> Option<Duration> {
    let mut result = Duration::from_secs(0);
    let mut current = String::new();
    let mut unit = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' {
            current.push(c);
        } else if c.is_alphabetic() {
            unit.push(c);
        } else if c.is_whitespace() && !current.is_empty() && !unit.is_empty() {
            if let Some(duration) = parse_duration_component(&current, &unit) {
                result += duration;
            }
            current.clear();
            unit.clear();
        }
    }

    if !current.is_empty() && !unit.is_empty() {
        if let Some(duration) = parse_duration_component(&current, &unit) {
            result += duration;
        }
    }

    Some(result)
}

/// Parses a duration component (e.g., "10s" or "5m")
fn parse_duration_component(value: &str, unit: &str) -> Option<Duration> {
    let value = value.parse::<f64>().ok()?;

    match unit {
        "h" | "hr" | "hrs" | "hour" | "hours" => Some(Duration::from_secs((value * 3600.0) as u64)),
        "m" | "min" | "mins" | "minute" | "minutes" => {
            Some(Duration::from_secs((value * 60.0) as u64))
        }
        "s" | "sec" | "secs" | "second" | "seconds" => Some(Duration::from_secs(value as u64)),
        "ms" | "millisecond" | "milliseconds" => Some(Duration::from_millis(value as u64)),
        "us" | "microsecond" | "microseconds" => Some(Duration::from_micros(value as u64)),
        "ns" | "nanosecond" | "nanoseconds" => Some(Duration::from_nanos(value as u64)),
        _ => None,
    }
}
