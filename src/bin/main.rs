use clap::Parser;
use lotka_volterra::{
    cli::Cli,
    interactive::interactive_mode,
    models::*,
    plot::*,
    solver::*,
    gui::launch_gui,
    error::SimulationError,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // If --gui is provided, skip the menu and go straight to the GUI
    if cli.gui {
        println!("🚀 Launching interactive GUI...");
        let params = LotkaVolterraParameters::default(); // Use default parameters
        launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
        return Ok(());
    }

    // Determine parameters: use CLI args if provided, otherwise enter interactive mode
    let params = if cli.interactive
        || cli.alpha.is_none()
        || cli.beta.is_none()
        || cli.delta.is_none()
        || cli.gamma.is_none()
    {
        match interactive_mode()? {
            Some(params) => params, // User selected "Use default parameters" or "Enter custom parameters"
            None => {
                // User selected "Interactive Plot"
                println!("🚀 Launching interactive GUI...");
                let params = LotkaVolterraParameters {
                    alpha: 0.0,
                    beta: 0.0,
                    delta: 0.0,
                    gamma: 0.0,
                    initial_prey: 0.0,
                    initial_predator: 0.0,
                };
                launch_gui(params).map_err(|e| SimulationError::GuiError(e.to_string()))?;
                return Ok(()); // Exit after launching the GUI
            }
        }
    } else {
        // Use CLI arguments
        let params = LotkaVolterraParameters {
            alpha: cli.alpha.unwrap(),
            beta: cli.beta.unwrap(),
            delta: cli.delta.unwrap(),
            gamma: cli.gamma.unwrap(),
            initial_prey: cli.initial_prey.unwrap(),
            initial_predator: cli.initial_predator.unwrap(),
        };
        params
    };

    // Run the simulation based on the selected mode
    if cli.interactive_plot {
        println!("🚀 Launching interactive GUI...");
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
