use crate::error::SimulationError;
use crate::models::LotkaVolterraParameters;
use crate::stochastic::launch_stochastic_gui;
use dialoguer::{Input, Select};
use std::error::Error;

/// Interactive mode displaying a menu for the user.
/// Returns `Ok(Some(params))` if the user selects "Use default parameters" or "Enter custom parameters".
/// Returns `Ok(None)` if the user selects "Interactive Plot" or "Interactive Stochastic Plot".
pub fn interactive_mode() -> Result<Option<LotkaVolterraParameters>, Box<dyn Error>> {
    println!("\nWelcome to the Lotka-Volterra Simulation CLI!");

    let choices = &[
        "Use default parameters",
        "Enter custom parameters",
        "Interactive Deterministic Plot",
        "Interactive Stochastic Plot",
    ];
    let selection = Select::new()
        .with_prompt("\nPlease select an option.")
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
                    .with_prompt("Enter prey death rate (beta)")
                    .interact_text()?,
                delta: Input::new()
                    .with_prompt("Enter predator birth rate (delta)")
                    .interact_text()?,
                gamma: Input::new()
                    .with_prompt("Enter predator death rate (gamma)")
                    .interact_text()?,
                initial_prey: Input::new()
                    .with_prompt("Enter initial prey population")
                    .interact_text()?,
                initial_predator: Input::new()
                    .with_prompt("Enter initial predator population")
                    .interact_text()?,
                t_start: Input::new()
                    .with_prompt("Enter simulation start time (t0)")
                    .interact_text()?,
                t_end: Input::new()
                    .with_prompt("Enter simulation end time (t_end)")
                    .interact_text()?,
            };
            validate_params(&params)?;
            Ok(Some(params))
        }
        2 => {
            // Interactive deterministic Plot; skips confirmation and return None
            Ok(None)
        }
        3 => {
            // Interactive stochastic plot
            println!("\nLaunching Stochastic Simulation GUI...\n");

            if let Err(e) = launch_stochastic_gui() {
                eprintln!("Error launching GUI: {}", e);
                return Err(Box::new(e));
            }

            // Ensure the program exits after the GUI closes
            std::process::exit(0);
        }
        // This should never happen
        _ => unreachable!(),
    }
}

/// Validates the parameters given for the Lotka-Volterra differential equation.
fn validate_params(params: &LotkaVolterraParameters) -> Result<(), SimulationError> {
    if params.alpha < 0.0
        || params.beta < 0.0
        || params.delta < 0.0
        || params.gamma < 0.0
        || params.initial_prey < 0.0
        || params.initial_predator < 0.0
        || params.t_start < 0.0
        || params.t_end < 0.0
        || params.t_end < params.t_start
    {
        return Err(SimulationError::InvalidParameter(
            "All parameters must be non-negative.".to_string(),
        ));
    }
    Ok(())
}
