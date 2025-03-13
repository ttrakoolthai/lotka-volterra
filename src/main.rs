use ode_solvers::{dopri5::Dopri5, System, SVector};
use plotters::prelude::*;

// Define parameters for the Lotka-Volterra system
const ALPHA: f64 = 0.1;  // Prey birth rate
const BETA: f64 = 0.02;  // Predation rate
const DELTA: f64 = 0.01; // Predator reproduction rate
const GAMMA: f64 = 0.1;  // Predator death rate

// Define the system of ODEs
type State = SVector<f64, 2>;

struct LotkaVolterra;

impl System<f64, State> for LotkaVolterra {
    fn system(&self, _t: f64, y: &State, dydt: &mut State) {
        dydt[0] = ALPHA * y[0] - BETA * y[0] * y[1]; // Prey dynamics
        dydt[1] = DELTA * y[0] * y[1] - GAMMA * y[1]; // Predator dynamics
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initial conditions: Prey = 40, Predator = 9
    let y0 = State::new(40.0, 9.0);
    let t0 = 0.0;
    let t_end = 200.0;
    let step = 0.1;

    // Create the solver
    let mut solver = Dopri5::new(LotkaVolterra, t0, t_end, step, y0, 1e-6, 1e-6);

    // Solve the system
    println!("Solving the system...");
    let result = solver.integrate();

    // Extract the solution data
    let (times, prey, predators) = match result {
        Ok(stats) => {
            println!("Integration succeeded.");
            // Access the solution data from the solver
            let times: Vec<f64> = solver.x_out().to_vec();
            let prey: Vec<f64> = solver.y_out().iter().map(|y| y[0]).collect();
            let predators: Vec<f64> = solver.y_out().iter().map(|y| y[1]).collect();
            (times, prey, predators)
        }
        Err(e) => {
            eprintln!("Integration failed: {e}");
            return Err(Box::new(e));
        }
    };

    // Debug: Print the data
    println!("Times: {:?}", times);
    println!("Prey: {:?}", prey);
    println!("Predators: {:?}", predators);

    // Plot the results
    println!("Plotting the results...");
    let root = BitMapBackend::new("lotka_volterra.png", (800, 600)).into_drawing_area();
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

    println!("Plot saved as lotka_volterra.png");
    Ok(())
}
