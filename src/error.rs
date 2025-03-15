use std::fmt;

/// Custom error type for the simulation.
#[derive(Debug)]
pub enum SimulationError {
    GuiError(String),
    InvalidParameter(String),
    PlotError(String),
    UserCancelled,
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationError::GuiError(msg) => write!(f, "GUI error: {}", msg),
            SimulationError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            SimulationError::PlotError(msg) => write!(f, "Plot error: {}", msg),
            SimulationError::UserCancelled => write!(f, "Simulation cancelled by user."),
        }
    }
}

impl std::error::Error for SimulationError {}
