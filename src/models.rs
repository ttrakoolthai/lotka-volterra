// src/models.rs (Define parameters & system behavior)
use ode_solvers::SVector;

#[derive(Debug, Clone, Copy)]
pub struct LotkaVolterraParameters {
    pub alpha: f64,              // Prey birth rate
    pub beta: f64,               // Predation rate
    pub delta: f64,              // Predator reproduction rate
    pub gamma: f64,              // Predator death rate
    pub initial_prey: f64,       // Initial prey population
    pub initial_predator: f64,    // Initial predator population
}

impl Default for LotkaVolterraParameters {
    fn default() -> Self {
        Self {
            alpha: 0.1,              // Prey birth rate
            beta: 0.02,               // Predation rate
            delta: 0.01,              // Predator reproduction rate
            gamma: 0.1,               // Predator death rate
            initial_prey: 100.0,      // Initial prey population
            initial_predator: 100.0,  // Initial predator population
        }
    }
}

pub type State = SVector<f64, 2>;

pub struct LotkaVolterraSystem {
    pub params: LotkaVolterraParameters,
}

impl LotkaVolterraSystem {
    pub fn new(params: LotkaVolterraParameters) -> Self {
        Self { params }
    }
}

impl ode_solvers::System<f64, State> for LotkaVolterraSystem {
    fn system(&self, _t: f64, y: &State, dydt: &mut State) {
        dydt[0] = self.params.alpha * y[0] - self.params.beta * y[0] * y[1];
        dydt[1] = self.params.delta * y[0] * y[1] - self.params.gamma * y[1];
    }
}
