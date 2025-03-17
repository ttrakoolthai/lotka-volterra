use dialoguer::{Confirm, Input, Select};
use crate::models::LotkaVolterraParameters;
use crate::error::SimulationError;
use std::error::Error;

/// Interactive mode for user input.
/// Returns `Ok(Some(params))` if the user selects "Use default parameters" or "Enter custom parameters".
/// Returns `Ok(None)` if the user selects "Interactive Plot".
pub fn interactive_mode() -> Result<Option<LotkaVolterraParameters>, Box<dyn Error>> {
    println!("🎯 Welcome to the Lotka-Volterra Simulation CLI!");

    let choices = &[
        "Use default parameters",
        "Enter custom parameters",
        "Interactive Plot",
    ];
    let selection = Select::new()
        .with_prompt("How would you like to proceed?")
        .default(0)
        .items(choices)
        .interact()
        .unwrap();

    match selection {
        0 => {
            // Use default parameters
            let params = LotkaVolterraParameters::default();
            validate_params(&params)?;
            Ok(Some(params))
        }
        1 => {
            // Enter custom parameters
            let params = LotkaVolterraParameters {
                alpha: Input::new()
                    .with_prompt("Enter prey birth rate (alpha)")
                    .interact_text()?,
                beta: Input::new()
                    .with_prompt("Enter predation rate (beta)")
                    .interact_text()?,
                delta: Input::new()
                    .with_prompt("Enter predator reproduction rate (delta)")
                    .interact_text()?,
                gamma: Input::new()
                    .with_prompt("Enter predator death rate (gamma)")
                    .interact_text()?,
                initial_prey: Input::new()
                    .with_prompt("Enter initial predator population")
                    .interact_text()?,
                initial_predator: Input::new()
                    .with_prompt("Enter initial predator population")
                    .interact_text()?,
                t_start: Input::new()
                    .with_prompt("Enter simulation start time (t0)") // 🔹 Fixed prompt
                    .interact_text()?,
                t_end: Input::new()
                    .with_prompt("Enter simulation end time (t_end)") // 🔹 Fixed prompt
                    .interact_text()?,
            };
            validate_params(&params)?;
            Ok(Some(params))
        }
        2 => {
            // Interactive Plot: Skip confirmation and return None
            Ok(None)
        }
        _ => unreachable!(), // This should never happen
    }
}

/// Validate Lotka-Volterra parameters.
fn validate_params(params: &LotkaVolterraParameters) -> Result<(), SimulationError> {
    if params.alpha < 0.0 || params.beta < 0.0 || params.delta < 0.0 || params.gamma < 0.0 || params.initial_prey < 0.0 || params.initial_predator < 0.0 || params.t_start < 0.0 || params.t_end < 0.0 || params.t_end < params.t_start {
        return Err(SimulationError::InvalidParameter(
            "All parameters must be non-negative.".to_string(),
        ));
    }
    Ok(())
}
