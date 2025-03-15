// solver.rs (Encapsulate solver logic)
// solve_lotka_volterra(): Handles ODE solving and returns results as Vec<(f64, f64, f64)> (time, prey, predators),  (Handles numerical solving)

use ode_solvers::SVector;
use crate::models::{LotkaVolterraParameters, LotkaVolterraSystem};
use ode_solvers::dopri5::Dopri5;
use std::error::Error;

pub fn solve_lotka_volterra(
    params: LotkaVolterraParameters,
    y0: [f64; 2],
    t0: f64,
    t_end: f64,
    step: f64,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let system = LotkaVolterraSystem::new(params);
    let mut solver = Dopri5::new(system, t0, t_end, step, SVector::from(y0), 1e-6, 1e-6);

    solver.integrate()?;
    Ok((
        solver.x_out().to_vec(),
        solver.y_out().iter().map(|y| y[0]).collect(),
        solver.y_out().iter().map(|y| y[1]).collect(),
    ))
}
