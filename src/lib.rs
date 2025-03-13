use ode_solvers::{dopri5::Dopri5, System, SVector};
use plotters::prelude::*;
use std::error::Error;

// Define parameters for the Lotka-Volterra system
#[derive(Debug, Clone, Copy)]
pub struct LotkaVolterraParams {
    pub alpha: f64, // Prey birth rate
    pub beta: f64,  // Predation rate
    pub delta: f64, // Predator reproduction rate
    pub gamma: f64, // Predator death rate
}

impl Default for LotkaVolterraParams {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            beta: 0.02,
            delta: 0.01,
            gamma: 0.1,
        }
    }
}

// Define the system of ODEs
type State = SVector<f64, 2>;

struct LotkaVolterraSystem {
    params: LotkaVolterraParams,
}

impl LotkaVolterraSystem {
    fn new(params: LotkaVolterraParams) -> Self {
        Self { params }
    }
}

impl System<f64, State> for LotkaVolterraSystem {
    fn system(&self, _t: f64, y: &State, dydt: &mut State) {
        dydt[0] = self.params.alpha * y[0] - self.params.beta * y[0] * y[1]; // Prey dynamics
        dydt[1] = self.params.delta * y[0] * y[1] - self.params.gamma * y[1]; // Predator dynamics
    }
}

/// Solves the Lotka-Volterra system and saves the plot to a file.
///
/// # Arguments
/// * `params` - Parameters for the Lotka-Volterra system.
/// * `y0` - Initial conditions: [prey, predator].
/// * `t0` - Initial time.
/// * `t_end` - End time.
/// * `step` - Step size for the solver.
/// * `output_file` - Path to save the plot.
///
/// # Returns
/// `Result<(), Box<dyn Error>>` - Result indicating success or failure.
pub fn plot_lotka_volterra(
    params: LotkaVolterraParams,
    y0: [f64; 2],
    t0: f64,
    t_end: f64,
    step: f64,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Create the solver
    let mut solver = Dopri5::new(
        LotkaVolterraSystem::new(params),
        t0,
        t_end,
        step,
        SVector::from(y0),
        1e-6,
        1e-6,
    );

    // Solve the system
    let result = solver.integrate();

    // Extract the solution data
    let (times, prey, predators) = match result {
        Ok(_) => {
            let times: Vec<f64> = solver.x_out().to_vec();
            let prey: Vec<f64> = solver.y_out().iter().map(|y| y[0]).collect();
            let predators: Vec<f64> = solver.y_out().iter().map(|y| y[1]).collect();
            (times, prey, predators)
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    // Plot the results
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Lotka-Volterra Predator-Prey Model", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..t_end, 0.0..50.0)?;

    chart.configure_mesh().draw()?;

    // Plot prey population
    chart
        .draw_series(LineSeries::new(
            times.iter().zip(prey.iter()).map(|(&x, &y)| (x, y)),
            &BLUE,
        ))?
        .label("Prey")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Plot predator population
    chart
        .draw_series(LineSeries::new(
            times.iter().zip(predators.iter()).map(|(&x, &y)| (x, y)),
            &RED,
        ))?
        .label("Predators")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Configure and draw the legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
