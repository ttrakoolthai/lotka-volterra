use std::fmt;

/// Custom error type for the Lotka-Volterra simulation.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_error() {
        let error = SimulationError::GuiError("GUI failed to launch".to_string());
        assert_eq!(format!("{}", error), "GUI error: GUI failed to launch");
    }

    #[test]
    fn test_invalid_parameter_error() {
        let error = SimulationError::InvalidParameter("Alpha cannot be negative".to_string());
        assert_eq!(
            format!("{}", error),
            "Invalid parameter: Alpha cannot be negative"
        );
    }

    #[test]
    fn test_plot_error() {
        let error = SimulationError::PlotError("Failed to render plot".to_string());
        assert_eq!(format!("{}", error), "Plot error: Failed to render plot");
    }

    #[test]
    fn test_user_cancelled() {
        let error = SimulationError::UserCancelled;
        assert_eq!(format!("{}", error), "Simulation cancelled by user.");
    }
}
