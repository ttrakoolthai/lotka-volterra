use ode_solvers::SVector;

/// The parameters required for solving the Lotka-Volterra differential equation.
#[derive(Debug, Clone, Copy)]
pub struct LotkaVolterraParameters {
    pub alpha: f64,            // Prey birth rate
    pub beta: f64,             // Prety death rate
    pub delta: f64,            // Predator birth rate
    pub gamma: f64,            // Predator death rate
    pub initial_prey: f64,     // Initial prey population
    pub initial_predator: f64, // Initial predator population
    pub t_start: f64,          // Starting time
    pub t_end: f64,            // Ending time
}

/// Defines parameters and system behavior.
impl Default for LotkaVolterraParameters {
    fn default() -> Self {
        Self {
            alpha: 0.01,
            beta: 0.00001,
            delta: 0.00001,
            gamma: 0.01,
            initial_prey: 2000.0,
            initial_predator: 2000.0,
            t_start: 0.0,
            t_end: 8000.0,
        }
    }
}

pub type State = SVector<f64, 2>;

pub struct LotkaVolterraSystem {
    pub params: LotkaVolterraParameters,
}

/// Creates the Lotka-Volterra system with the provided parameters.
impl LotkaVolterraSystem {
    pub fn new(params: LotkaVolterraParameters) -> Self {
        Self { params }
    }
}

/// Solves the differential equation.
impl ode_solvers::System<f64, State> for LotkaVolterraSystem {
    fn system(&self, _t: f64, y: &State, dydt: &mut State) {
        dydt[0] = self.params.alpha * y[0] - self.params.beta * y[0] * y[1];
        dydt[1] = self.params.delta * y[0] * y[1] - self.params.gamma * y[1];
    }
}
