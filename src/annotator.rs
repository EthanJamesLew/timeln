use crate::time_formatter::{TimeFormat};
use std::time::Duration;

use colored::Colorize;

/// The `TimelnAnnotation` trait provides an abstraction over line annotation.
/// Implementations of `TimelnAnnotation` provide methods to format lines with timestamps and time deltas.
pub trait TimelnAnnotation {
    /// Takes a line of text, the current time, and time delta and formats it into a `String`.
    ///
    /// # Arguments
    ///
    /// * `line` - A line of text to be annotated.
    /// * `now` - A `Duration` instance representing the current time.
    /// * `delta` - A `Duration` instance representing the time delta.
    ///
    /// # Returns
    ///
    /// * `String` - A string with the line text and its annotated information.
    fn format_line(&self, line: &String, now: &Duration, delta: &Duration) -> String; 
}

/// The `SimpleAnnotator` struct is an implementation of the `TimelnAnnotation` trait that annotates lines with simple time and delta information.
pub struct SimpleAnnotator {
    pub color: bool,
    pub time_format: Box<dyn TimeFormat>,
}

impl TimelnAnnotation for SimpleAnnotator {
    /// Annotates the given line with the current time and delta in a simple format.
    fn format_line(&self, line: &String, now: &Duration, delta: &Duration) -> String {
        let time_str = self.time_format.format_duration(now);
        let delta_str = self.time_format.format_duration(delta);
        
        if self.color {
            let color_annotation = format!("[time: {}, delta: {}]", time_str, delta_str);
            format!("{} {}", color_annotation.green(), line)
        } else {
            format!("[time: {}, delta: {}] {}", time_str, delta_str, line)
        }
    }
}

/// The `UnicodeAnnotator` struct is an implementation of the `TimelnAnnotation` trait that annotates lines with Unicode symbols for time and delta.
pub struct UnicodeAnnotator {
    pub color: bool,
    pub time_format: Box<dyn TimeFormat>,
}

impl TimelnAnnotation for UnicodeAnnotator {
    /// Annotates the given line with the current time and delta in a Unicode format.
    fn format_line(&self, line: &String, now: &Duration, delta: &Duration) -> String {
        let time_str = self.time_format.format_duration(now);
        let delta_str = self.time_format.format_duration(delta);
        
        if self.color {
            let color_annotation = format!("[Τ: {}, Δ: {}]", time_str, delta_str);
            format!("{} {}", color_annotation.green(), line)
        } else {
            format!("[Τ: {}, Δ: {}] {}", time_str, delta_str, line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_formatter::SecondsFormat;

    #[test]
    fn test_simple_annotation() {
        let annotator = SimpleAnnotator { color: false, time_format: Box::new(SecondsFormat) };
        let now = Duration::new(5, 500_000_000); // 5.5 seconds
        let delta = Duration::new(1, 500_000_000); // 1.5 seconds
        let line = "Sample line".to_string();
        assert_eq!(
            annotator.format_line(&line, &now, &delta),
            "[time: 5.50 s, delta: 1.50 s] Sample line"
        );
    }

    #[test]
    fn test_unicode_annotator() {
        let annotator = UnicodeAnnotator { color: false, time_format: Box::new(SecondsFormat) };
        let now = Duration::new(5, 500_000_000); // 5.5 seconds
        let delta = Duration::new(1, 500_000_000); // 1.5 seconds
        let line = "Sample line".to_string();
        assert_eq!(
            annotator.format_line(&line, &now, &delta),
            "[Τ: 5.50 s, Δ: 1.50 s] Sample line"
        );
    }
}
