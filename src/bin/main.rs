use clap::Parser;
use dialoguer::{Confirm, Input, Select};
use lotka_volterra::{models::*, plot::*, solver::*, gui::*};
use std::error::Error;

/// Custom error type for the simulation.
#[derive(Debug)]
enum SimulationError {
    InvalidParameter(String),
    UserCancelled,
    PlotError(String),
    GuiError(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            SimulationError::UserCancelled => write!(f, "Simulation cancelled by user."),
            SimulationError::PlotError(msg) => write!(f, "Plot error: {}", msg),
            SimulationError::GuiError(msg) => write!(f, "GUI error: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

/// Command-line arguments for Lotka-Volterra simulation.
#[derive(Parser, Debug)]
#[command(name = "Lotka-Volterra Simulator")]
#[command(about = "Simulate predator-prey dynamics", long_about = None)]
struct Cli {
    /// Prey birth rate (alpha)
    #[arg(long)]
    alpha: Option<f64>,

    /// Predation rate (beta)
    #[arg(long)]
    beta: Option<f64>,

    /// Predator reproduction rate (delta)
    #[arg(long)]
    delta: Option<f64>,

    /// Predator death rate (gamma)
    #[arg(long)]
    gamma: Option<f64>,

    /// Run in interactive mode
    #[arg(long)]
    interactive: bool,

    /// Launch the interactive GUI
    #[arg(long)]
    gui: bool,

    /// Enable interactive plot mode
    #[arg(long)]
    interactive_plot: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Determine parameters: use CLI args if provided, otherwise enter interactive mode
    let params = if cli.interactive
        || cli.alpha.is_none()
        || cli.beta.is_none()
        || cli.delta.is_none()
        || cli.gamma.is_none()
    {
        interactive_mode()?
    } else {
        let params = LotkaVolterraParameters {
            alpha: cli.alpha.unwrap(),
            beta: cli.beta.unwrap(),
            delta: cli.delta.unwrap(),
            gamma: cli.gamma.unwrap(),
        };
        validate_params(&params)?; // Validate parameters
        params
    };

    // Run the simulation based on the selected mode
    if cli.gui {
        println!("🚀 Launching interactive GUI...");
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
    } else if cli.interactive_plot {
        println!("📊 Launching Interactive Plot...");
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
    } else {
        println!("✅ Running simulation...");

        // Initial conditions: [prey, predator]
        let y0 = [40.0, 9.0];
        let t0 = 0.0;
        let t_end = 200.0;
        let step = 0.1;

        // Run the simulation
        let (times, prey, predators) = solve_lotka_volterra(params, y0, t0, t_end, step)
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        // Plot results
        plot_lotka_volterra(&times, &prey, &predators, "lotka_volterra.png")
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        println!("📊 Plot saved as lotka_volterra.png");
    }

    Ok(())
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

/// Interactive mode for user input.
fn interactive_mode() -> Result<LotkaVolterraParameters, Box<dyn Error>> {
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

/// Launch the interactive GUI.
fn launch_gui(params: LotkaVolterraParameters) -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lotka-Volterra Model",
        options,
        Box::new(|_cc| Ok(Box::new(LotkaVolterraApp::new(params)))),
    )
    .map_err(|e| SimulationError::GuiError(e.to_string()).into())
}
