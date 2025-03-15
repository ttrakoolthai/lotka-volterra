// models.rs (Define parameters & system behavior)
// LotkaVolterraParameters
// LotkaVolterraSystem
// Implements System<f64, State>,  (Defines Lotka-Volterra parameters & system)

// src/models.rs
use ode_solvers::SVector;

#[derive(Debug, Clone, Copy)]
pub struct LotkaVolterraParameters {
    pub alpha: f64,
    pub beta: f64,
    pub delta: f64,
    pub gamma: f64,
}

impl Default for LotkaVolterraParameters {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            beta: 0.02,
            delta: 0.01,
            gamma: 0.1,
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
