use plotters::prelude::*;
use std::error::Error;

/// Statically plots the solution of the Lotka-Volterra system.
pub fn plot_lotka_volterra(
    times: &[f64],
    prey: &[f64],
    predators: &[f64],
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Lotka-Volterra Predator-Prey Model", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..times.last().copied().unwrap_or(1.0), 0.0..3000.0)?;
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(
            times.iter().zip(prey.iter()).map(|(&x, &y)| (x, y)),
            &BLUE,
        ))?
        .label("Prey")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
    chart
        .draw_series(LineSeries::new(
            times.iter().zip(predators.iter()).map(|(&x, &y)| (x, y)),
            &RED,
        ))?
        .label("Predators")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    chart.configure_series_labels().draw()?;
    Ok(())
}
