use dialoguer::{Confirm, Input, Select};
use crate::models::LotkaVolterraParameters;
use crate::error::SimulationError;
use std::error::Error;

/// Interactive mode for user input.
pub fn interactive_mode() -> Result<LotkaVolterraParameters, Box<dyn Error>> {
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

    let params = if selection == 0 {
        LotkaVolterraParameters::default()
    } else if selection == 1 {
        LotkaVolterraParameters {
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
        }
    } else {
        LotkaVolterraParameters::default()
    };

    // Validate parameters
    validate_params(&params)?;

    let confirm = Confirm::new()
        .with_prompt("Start the simulation with these parameters?")
        .default(true)
        .interact()
        .unwrap();

    if !confirm {
        println!("🚫 Simulation canceled.");
        return Err(Box::new(SimulationError::UserCancelled));
    }

    Ok(params)
}

/// Validate Lotka-Volterra parameters.
fn validate_params(params: &LotkaVolterraParameters) -> Result<(), SimulationError> {
    if params.alpha < 0.0 || params.beta < 0.0 || params.delta < 0.0 || params.gamma < 0.0 {
        return Err(SimulationError::InvalidParameter(
            "All parameters must be non-negative.".to_string(),
        ));
    }
    Ok(())
}
