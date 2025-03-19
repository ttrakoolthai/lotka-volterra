use crate::models::{LotkaVolterraParameters, LotkaVolterraSystem};
use ode_solvers::SVector;
use ode_solvers::dopri5::Dopri5;

/// Solves the deterministic Lotka-Volterra system using the ode_solver crate.
type SolverResult = Result<(Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn std::error::Error>>;

pub fn solve_lotka_volterra(
    params: LotkaVolterraParameters,
    y0: [f64; 2],
    t0: f64,
    t_end: f64,
    step: f64,
) -> SolverResult {
    let system = LotkaVolterraSystem::new(params);
    let mut solver = Dopri5::new(system, t0, t_end, step, SVector::from(y0), 1e-6, 1e-6);

    solver.integrate()?;
    Ok((
        solver.x_out().to_vec(),
        solver.y_out().iter().map(|y| y[0]).collect(),
        solver.y_out().iter().map(|y| y[1]).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_solver_runs() {
        let params = LotkaVolterraParameters {
            alpha: 0.1,
            beta: 0.02,
            gamma: 0.02,
            delta: 0.1,
            initial_prey: 40.0,
            initial_predator: 9.0,
            t_start: 0.0,
            t_end: 200.0,
        };

        let y0 = [params.initial_prey, params.initial_predator];
        let t_start = 0.0;
        let t_end = 200.0;
        let step = 0.1;

        let result = solve_lotka_volterra(params, y0, t_start, t_end, step);

        assert!(
            result.is_ok(),
            "Deterministic solver should not return an error."
        );
    }
}
