use std::time::Duration;
use crate::time_format::TimeFormat;

pub trait Summarizer: Sync + Send {
    fn summarize(&self, total_lines: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String;
}

pub struct SimpleSummarizer;

impl Summarizer for SimpleSummarizer {
    fn summarize(&self, total_lines: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String {
        let time_str = time_format.format_duration(total_time);
        format!("Processed {} lines in {}", total_lines, time_str)
    }
}

pub struct DetailedSummarizer;

impl Summarizer for DetailedSummarizer {
    fn summarize(&self, total_lines: usize, total_time: &Duration, time_format: &dyn TimeFormat) -> String {
        let time_str = time_format.format_duration(total_time);
        let avg_time_per_line = if total_lines > 0 {
            let total_ns = total_time.as_nanos() as u64;
            let avg_ns = total_ns / total_lines as u64;
            Duration::from_nanos(avg_ns)
        } else {
            Duration::default()
        };
        let avg_time_str = time_format.format_duration(&avg_time_per_line);
        format!("Processed {} lines in {}. Average time per line: {}", total_lines, time_str, avg_time_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_simple_summarizer() {
        let summarizer: Box<dyn Summarizer> = Box::new(SimpleSummarizer);
        let time_format: Box<dyn TimeFormat> = Box::new(SecondsFormat);
        let total_lines = 100;
        let total_time = Duration::new(30, 0); // 30 seconds
        let summary = summarizer.summarize(total_lines, &total_time, &*time_format);
        assert_eq!(summary, "Processed 100 lines in 30.00 s");
    }

    #[test]
    fn test_detailed_summarizer() {
        let summarizer: Box<dyn Summarizer> = Box::new(DetailedSummarizer);
        let time_format: Box<dyn TimeFormat> = Box::new(SecondsFormat);
        let total_lines = 100;
        let total_time = Duration::new(100, 0); // 100 seconds
        let summary = summarizer.summarize(total_lines, &total_time, &*time_format);
        assert_eq!(summary, "Processed 100 lines in 100.00 s. Average time per line: 1.00 s");
    }
}
