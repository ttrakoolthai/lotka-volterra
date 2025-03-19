use clap::Parser;
use lotka_volterra::{
    cli::Cli, error::SimulationError, gui::launch_gui, interactive::interactive_mode, models::*,
    plot::*, solver::*,
};
use std::error::Error;

/// Parses the command-line for the arguments needed to solve the Lotka-Volterra
/// differential equation. By default, the program launches an interactive menu
/// when no command-line arguments are supplied. The program will save the plot
/// as a graph as a png in the directory of the project.
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Skips interactive menu if program is run with the GUI option
    if cli.gui {
        println!("\nLaunching interactive GUI...");

        // GUI begins with default parameters
        let params = LotkaVolterraParameters::default();
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;

        return Ok(());
    }

    // Determine parameters for CLI arguments if provided; use interactive mode otherwise
    let params = if cli.interactive
        || cli.alpha.is_none()
        || cli.beta.is_none()
        || cli.delta.is_none()
        || cli.gamma.is_none()
        || cli.initial_prey.is_none()
        || cli.initial_predator.is_none()
        || cli.t_start.is_none()
        || cli.t_end.is_none()
    {
        match interactive_mode()? {
            // User selected "Use default parameters" or "Enter custom parameters"
            Some(params) => params,
            None => {
                // User selected "Interactive Plot"
                println!("\nLaunching interactive GUI...");
                let params = LotkaVolterraParameters {
                    alpha: 0.01,
                    beta: 0.00001,
                    delta: 0.00001,
                    gamma: 0.01,
                    initial_prey: 2000.0,
                    initial_predator: 2000.0,
                    t_start: 0.0,
                    t_end: 8000.0,
                };
                launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
                return Ok(());
            }
        }
    } else {
        // Use CLI arguments
        LotkaVolterraParameters {
            alpha: cli.alpha.unwrap(),
            beta: cli.beta.unwrap(),
            delta: cli.delta.unwrap(),
            gamma: cli.gamma.unwrap(),
            initial_prey: cli.initial_prey.unwrap(),
            initial_predator: cli.initial_predator.unwrap(),
            t_start: cli.t_start.unwrap(),
            t_end: cli.t_end.unwrap(),
        }
    };

    // Run simulation with selected mode
    if cli.interactive_plot {
        println!("\nLaunching interactive GUI...");
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
    } else {
        println!("\nRunning simulation...");

        // Initial conditions
        let y0 = [2000.0, 2000.0];
        let t0 = 0.0;
        let t_end = 8000.0;
        let step = 0.1;

        // Run the simulation
        let (times, prey, predators) = solve_lotka_volterra(params, y0, t0, t_end, step)
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        // Plot results
        plot_lotka_volterra(&times, &prey, &predators, "lotka_volterra.png")
            .map_err(|e| SimulationError::PlotError(e.to_string()))?;

        println!("Plot saved as lotka_volterra.png");
    }

    Ok(())
}
