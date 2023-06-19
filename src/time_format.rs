use std::time::Duration;

/// The `TimeFormat` trait provides an abstraction over formatting of `Duration`s.
/// Implementations of `TimeFormat` provide methods to convert `Duration` into a human-readable string format.
pub trait TimeFormat {
    /// Takes a `Duration` and formats it into a `String`.
    ///
    /// # Arguments
    ///
    /// * `duration` - A `Duration` instance representing a span of time.
    ///
    /// # Returns
    ///
    /// * `String` - A string representation of the `Duration` instance in a specific format.
    fn format_duration(&self, duration: &Duration) -> String;
}

/// The `SecondsFormat` struct is an implementation of the `TimeFormat` trait that formats durations as seconds.
#[derive(Debug, Clone, Copy)]
pub struct SecondsFormat;

impl TimeFormat for SecondsFormat {
    /// Takes a `Duration` and formats it into a `String` representation of seconds.
    fn format_duration(&self, duration: &Duration) -> String {
        let in_seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
        format!("{:.2} s", in_seconds)
    }
}

/// The `MillisecondsFormat` struct is an implementation of the `TimeFormat` trait that formats durations as milliseconds.
#[derive(Debug, Clone, Copy)]
pub struct MillisecondsFormat;

impl TimeFormat for MillisecondsFormat {
    /// Takes a `Duration` and formats it into a `String` representation of milliseconds.
    fn format_duration(&self, duration: &Duration) -> String {
        let in_milliseconds = duration.as_secs() as f64 * 1e3 + duration.subsec_nanos() as f64 * 1e-6;
        format!("{:.2} ms", in_milliseconds)
    }
}

/// The `MinutesSecondsFormat` struct is an implementation of the `TimeFormat` trait that formats durations as a combination of minutes and seconds.
#[derive(Debug, Clone, Copy)]
pub struct MinutesSecondsFormat;

impl TimeFormat for MinutesSecondsFormat {
    /// Takes a `Duration` and formats it into a `String` representation of minutes and seconds.
    fn format_duration(&self, duration: &Duration) -> String {
        let in_seconds = duration.as_secs();
        let minutes = in_seconds / 60;
        let seconds = in_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_format() {
        let format = SecondsFormat;
        let duration = Duration::new(5, 500_000_000); // 5.5 seconds
        assert_eq!(format.format_duration(&duration), "5.50 s");
    }

    #[test]
    fn test_milliseconds_format() {
        let format = MillisecondsFormat;
        let duration = Duration::new(5, 500_000_000); // 5.5 seconds
        assert_eq!(format.format_duration(&duration), "5500.00 ms");
    }

    #[test]
    fn test_minutes_seconds_format() {
        let format = MinutesSecondsFormat;
        let duration = Duration::new(125, 0); // 125 seconds = 2 minutes and 5 seconds
        assert_eq!(format.format_duration(&duration), "2m 5s");
    }
}
