use std::fmt;

/// Custom error type for the simulation.
#[derive(Debug)]
pub enum SimulationError {
    InvalidParameter(String),
    UserCancelled,
    PlotError(String),
    GuiError(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            SimulationError::UserCancelled => write!(f, "Simulation cancelled by user."),
            SimulationError::PlotError(msg) => write!(f, "Plot error: {}", msg),
            SimulationError::GuiError(msg) => write!(f, "GUI error: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}
