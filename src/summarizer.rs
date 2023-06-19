use std::time::Duration;
use colored::Colorize;
use crate::time_formatter::TimeFormat;

/// A trait for objects that can summarize a process by providing a summary string
/// based on total lines processed, total time taken, and a specified time format.
pub trait Summarizer: Sync + Send {
    /// Summarize the total lines processed and the total time taken in the specified format.
    ///
    /// # Arguments
    ///
    /// * `total_lines` - The total number of lines processed.
    /// * `total_time` - The total time taken to process lines.
    /// * `time_format` - The format to display time.
    ///
    /// # Returns
    ///
    /// A string containing the summary of the process.
    fn summarize(&self, total_lines: usize, total_matches: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String;
}

/// A simple implementation of the `Summarizer` trait.
pub struct SimpleSummarizer {
    pub color: bool,
}

impl Summarizer for SimpleSummarizer {
    fn summarize(&self, total_lines: usize, total_matches: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String {
        let time_str = time_format.format_duration(total_time);
        if self.color {
            format!("[Processed Lines: {}, Matches: {}, Total Time: {}]", total_lines, total_matches, time_str).green().to_string()
        } else {
            format!("[Processed Lines: {}, Matches: {}, Total Time: {}]", total_lines, total_matches, time_str)
        }
    }
}

/// A detailed implementation of the `Summarizer` trait that also provides an average time per line.
pub struct DetailedSummarizer{
    pub color: bool,
}

impl Summarizer for DetailedSummarizer {
    fn summarize(&self, total_lines: usize, total_matches: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String {
        let time_str = time_format.format_duration(total_time);
        let avg_time_per_line = if total_lines > 0 {
            let total_ns = total_time.as_nanos() as u64;
            let avg_ns = total_ns / total_lines as u64;
            Duration::from_nanos(avg_ns)
        } else {
            Duration::default()
        };
        let avg_time_str = time_format.format_duration(&avg_time_per_line);
        if self.color {
            format!("Processed {} lines in {} with {} matches. Average time per line: {}", total_lines, time_str, total_matches, avg_time_str).green().to_string()
        } else {
            format!("Processed {} lines in {} with {} matches. Average time per line: {}", total_lines, time_str, total_matches, avg_time_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_formatter::SecondsFormat;
    use std::time::Duration;

    #[test]
    fn test_simple_summarizer() {
        let summarizer: Box<dyn Summarizer> = Box::new(SimpleSummarizer { color: false });
        let time_format: Box<dyn TimeFormat> = Box::new(SecondsFormat);
        let total_lines = 100;
        let total_time = Duration::new(30, 0); // 30 seconds
        let summary = summarizer.summarize(total_lines, 0, &total_time, &*time_format);
        assert_eq!(summary, "[Processed Lines: 100, Matches: 0, Total Time: 30.00 s]");
    }

    #[test]
    fn test_detailed_summarizer() {
        let summarizer: Box<dyn Summarizer> = Box::new(DetailedSummarizer { color: false });
        let time_format: Box<dyn TimeFormat> = Box::new(SecondsFormat);
        let total_lines = 100;
        let total_time = Duration::new(100, 0); // 100 seconds
        let summary = summarizer.summarize(total_lines, 0, &total_time, &*time_format);
        assert_eq!(summary, "Processed 100 lines in 100.00 s with 0 matches. Average time per line: 1.00 s");
    }
}
