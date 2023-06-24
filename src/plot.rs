use plotters::prelude::*;

/// Plots time deltas and saves the plot as a PNG file.
///
/// This function takes a vector of time deltas in seconds and a filename as arguments.
/// It creates a line plot of these deltas and saves it as a PNG file.
/// The x-axis of the plot represents the line number, and the y-axis represents the time delta.
/// The plot also includes a title and labels for both axes.
///
/// # Arguments
///
/// * `deltas` - A vector of f64 values representing time deltas in seconds.
/// * `filename` - The name of the file (including the extension) where the plot should be saved.
///
/// # Errors
///
/// This function will return an error if the file cannot be created or written to.
///
/// # Example
///
/// ```
/// let deltas = vec![0.1, 0.2, 0.3, 0.4, 0.5];
/// let filename = "deltas.png";
/// plot_deltas(&deltas, filename).unwrap();
/// ```
pub fn plot_deltas(deltas: &Vec<f64>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = *deltas
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&1f64);
    let min_y = 0f64;
    let max_x = deltas.len() as f64;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Line number vs Time delta", ("Arial", 30).into_font())
        .set_all_label_area_size(50)
        .build_cartesian_2d(0f64..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc("Line number")
        .y_desc("Time delta (seconds)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        deltas.iter().enumerate().map(|(x, y)| (x as f64, *y)),
        &RED,
    ))?;

    Ok(())
}

pub fn plot_times(times: &Vec<f64>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = *times
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&1f64);
    let min_y = 0f64;
    let max_x = times.len() as f64;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Line number vs Time Elapsed", ("Arial", 30).into_font())
        .set_all_label_area_size(50)
        .build_cartesian_2d(0f64..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc("Line number")
        .y_desc("Time Elapsed (seconds)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        times.iter().enumerate().map(|(x, y)| (x as f64, *y)),
        &BLUE,
    ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_plot_deltas() -> Result<(), Box<dyn std::error::Error>> {
        let deltas = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let filename = "test_deltas.svg";
        plot_deltas(&deltas, filename)?;

        // Check that the file was created
        assert!(Path::new(filename).exists());

        // Check that the file is not empty
        let metadata = std::fs::metadata(filename)?;
        assert!(metadata.len() > 0);

        // Cleanup
        std::fs::remove_file(filename)?;

        Ok(())
    }
}
